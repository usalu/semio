/**
 * 🧪️ Wave Z depth: the cases of the new authored parameters (stair construction, railing sections, baluster rows and infill, sill
 * override, plan cut height). Each case is derived from an existing case of its leaf (its `before` snapshot) with one edited
 * payload. Usage: `bun r7-z-depth-new-cases.ts`. Re-running refuses (a case of that name exists). Applied cases need `BIM_BLESS=1`.
 */
import { addCase, caseDirs, read, type Outcome } from "./r6-z-mutations-cases.ts";

const json = (value: unknown) => JSON.stringify(value, null, 2) + "\n";
const reject = (code: string, path: string[]): Outcome => ({ status: "rejected", code, path });
const ok: Outcome = { status: "applied" };

const dirOf = (kind: string, name: string) => {
  const found = caseDirs(kind).find((dir) => dir.endsWith(name));
  if (!found) throw new Error(`${kind} has no case ${name}`);
  return found;
};
const baseOf = (kind: string, name: string) => JSON.parse(read(kind, dirOf(kind, name), "before"));
const payloadOf = (kind: string, name: string) => JSON.parse(read(kind, dirOf(kind, name), "mutation"));

function derive(kind: string, from: string, name: string, outcome: Outcome, edit: (payload: any, before: any) => void, tweak: (before: any) => void = () => {}) {
  const before = baseOf(kind, from);
  tweak(before);
  const payload = payloadOf(kind, from);
  edit(payload, before);
  addCase(kind, name, { beforeText: json(before), mutationText: json(payload), outcome });
}

const construct = { stringer: { kind: "Open", width: 0.05, depth: 0.22 }, nosing: 0.03, tread_thickness: 0.05, riser: "Open", landing_depth: 1.1 };

derive("create-stair", "adds-a-straight-flight", "adds-an-open-stair-with-cut-stringers", ok, (p) => Object.assign(p.stair, construct));
derive("create-stair", "adds-a-straight-flight", "stringer-without-size", reject("mutation.invariant", ["stair", "stringer"]), (p) => (p.stair.stringer = { kind: "Closed", width: 0, depth: 0.2 }));
derive("create-stair", "adds-a-straight-flight", "nosing-longer-than-the-tread", reject("mutation.invariant", ["stair", "nosing"]), (p) => (p.stair.nosing = 0.3));
derive("create-stair", "adds-a-straight-flight", "non-positive-tread-thickness", reject("mutation.invariant", ["stair", "tread_thickness"]), (p) => (p.stair.tread_thickness = 0));
derive("create-stair", "adds-a-straight-flight", "non-positive-landing-depth", reject("mutation.invariant", ["stair", "landing_depth"]), (p) => (p.stair.landing_depth = 0));

derive("set-stair", "moves-and-widens", "builds-open-risers-and-a-mono-stringer", ok, (p) => {
  p.stringer = { kind: "Mono", width: 0.12, depth: 0.3 };
  p.riser = "Open";
  p.nosing = 0.03;
  p.tread_thickness = 0.05;
  p.landing_depth = 1.2;
  delete p.start;
  delete p.direction;
  delete p.width;
});
derive("set-stair", "moves-and-widens", "nosing-longer-than-the-tread", reject("mutation.invariant", ["nosing"]), (p) => {
  for (const key of ["start", "direction", "width"]) delete p[key];
  p.nosing = 0.4;
});
derive("set-stair", "moves-and-widens", "stringer-without-size", reject("mutation.invariant", ["stringer"]), (p) => {
  for (const key of ["start", "direction", "width"]) delete p[key];
  p.stringer = { kind: "Closed", width: 0, depth: 0.2 };
});

const glass = { profile: { Circle: { diameter: 0.05 } }, post_profile: { Rectangle: { width: 0.06, depth: 0.06 } }, baluster: { profile: { Rectangle: { width: 0.02, depth: 0.02 } }, spacing: 0.12 }, infill: { Glass: { thickness: 0.012 } } };
derive("create-railing", "adds", "adds-balusters-and-a-glass-infill", ok, (p) => Object.assign(p.railing, glass));
derive("create-railing", "adds", "baluster-spacing-not-positive", reject("mutation.invariant", ["railing", "baluster"]), (p) => (p.railing.baluster = { profile: { Rectangle: { width: 0.02, depth: 0.02 } }, spacing: 0 }));
derive("create-railing", "adds", "infill-thickness-not-positive", reject("mutation.invariant", ["railing", "infill"]), (p) => (p.railing.infill = { Panel: { thickness: 0 } }));
derive("create-railing", "adds", "rail-profile-degenerate", reject("mutation.invariant", ["railing", "profile"]), (p) => (p.railing.profile = { Rectangle: { width: 0, depth: 0.04 } }));

derive("set-railing", "reshapes", "adds-balusters-and-glass", ok, (p) => {
  for (const key of ["path", "height", "material"]) delete p[key];
  p.profile = glass.profile;
  p.baluster = { value: glass.baluster };
  p.infill = glass.infill;
});
derive(
  "set-railing",
  "reshapes",
  "removes-the-balusters",
  ok,
  (p) => {
    for (const key of ["path", "height", "material"]) delete p[key];
    p.baluster = { value: null };
  },
  (before) => (before.railings["rl-1"].baluster = glass.baluster),
);
derive("set-railing", "reshapes", "post-profile-degenerate", reject("mutation.invariant", ["post_profile"]), (p) => {
  for (const key of ["path", "height", "material"]) delete p[key];
  p.post_profile = { Circle: { diameter: 0 } };
});

derive("create-storey", "stacks-above", "with-a-plan-cut-height", ok, (p) => (p.storey.cut_height = 1.5));
derive("create-storey", "stacks-above", "non-positive-plan-cut-height", reject("mutation.invariant", ["storey", "cut_height"]), (p) => (p.storey.cut_height = 0));

derive("create-opening", "adds-a-window", "adds-a-window-with-a-raised-sill", ok, (p) => (p.opening.sill_override = 1.2));
derive("set-opening", "resizes", "sets-a-sill-override", ok, (p) => {
  delete p.width;
  delete p.height;
  p.sill_override = { value: 1.2 };
}, (before) => delete before.openings["o-1"].sill_override);
derive("set-opening", "resizes", "restores-the-type-sill", ok, (p) => {
  delete p.width;
  delete p.height;
  p.sill_override = { value: null };
});
