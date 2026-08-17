// 🧪️ Standalone smoke verification for the new kernel `🔖️AppRouter`/`🔖️OpeningResolver` regions —
// glue.ts's vitest `includeSource` is scoped to itself only (see 🧪️vitest.config.ts), so these new
// kernel exports get zero coverage from `bun nx run @semio-tech/framework:test` until glue.ts's own
// test block is extended (out of this lane's lease — flagged as a follow-up). Run once, ad hoc:
//   bun run 🧪️w1-b-verify.ts
import {
  AppRouter,
  EMPTY_OPENING_PREFERENCES,
  SURFACE_FAULT_CODES,
  decodeOpeningConfigMutation,
  decodeOpeningPreferences,
  dialectCoordinate,
  foldOpeningPreferences,
  parseDialectCoordinate,
  parseSurfaceAppId,
  resolveOpeningApp,
  surfaceAppId,
  type AppRouterManifest,
  type ArtifactDialect,
} from "../../../../../../🧰️framework/🔨️modules/🎠️kernel/🟦️component.ts";

let failures = 0;
function check(label: string, actual: unknown, expected: unknown): void {
  const a = JSON.stringify(actual);
  const e = JSON.stringify(expected);
  if (a !== e) {
    failures += 1;
    console.error(`[FAIL] ${label}: got ${a}, expected ${e}`);
  } else {
    console.log(`[ok] ${label}`);
  }
}
function checkThrows(label: string, fn: () => void, codeSubstring: string): void {
  try {
    fn();
    failures += 1;
    console.error(`[FAIL] ${label}: expected throw, got none`);
  } catch (error) {
    const message = error instanceof Error ? error.message : String(error);
    const code = (error as { fault?: { code?: string } }).fault?.code;
    if (code === codeSubstring || message.includes(codeSubstring)) {
      console.log(`[ok] ${label}`);
    } else {
      failures += 1;
      console.error(`[FAIL] ${label}: wrong error ${message} (fault.code=${code})`);
    }
  }
}

//#region 🪪️Coordinate/SurfaceId round trips
const cad: ArtifactDialect = { artifactKind: "s.cad.cad", standard: "1", subset: "*" };
check("dialectCoordinate", dialectCoordinate(cad), "s.cad.cad@1/*");
check("parseDialectCoordinate", parseDialectCoordinate("s.cad.cad@1/*"), cad);
check("surfaceAppId editor", surfaceAppId(cad, "editor"), "s.cad.cad@1/*#editor");
check("parseSurfaceAppId editor", parseSurfaceAppId("s.cad.cad@1/*#editor"), { dialect: cad, role: "editor" });
const dotted: ArtifactDialect = { artifactKind: "s.stdio.gif", standard: "1.7", subset: "any" };
check("dotted standard round trip", parseDialectCoordinate(dialectCoordinate(dotted)), dotted);
//#endregion

//#region 🧭️AppRouter ordering + owner-first
const manifests: AppRouterManifest[] = [
  {
    pluginId: "cad",
    artifactKinds: [{ id: "s.cad.cad" }],
    apps: [
      { id: surfaceAppId(cad, "editor"), role: "editor", dialect: cad },
      { id: surfaceAppId(cad, "viewer"), role: "viewer", dialect: cad },
    ],
  },
  {
    pluginId: "aec-building",
    dependencies: [{ pluginId: "cad", version: "*" }],
    apps: [{ id: surfaceAppId(cad, "editor"), role: "editor", dialect: cad }],
  },
  {
    pluginId: "aaa-first",
    dependencies: [{ pluginId: "cad", version: "*" }],
    apps: [{ id: surfaceAppId(cad, "editor"), role: "editor", dialect: cad }],
  },
];
const router = AppRouter.build(manifests);
check("owner first, then pluginId ascending", router.entriesFor(cad, "editor"), [
  { pluginId: "cad", appId: surfaceAppId(cad, "editor") },
  { pluginId: "aaa-first", appId: surfaceAppId(cad, "editor") },
  { pluginId: "aec-building", appId: surfaceAppId(cad, "editor") },
]);
check("ownerPluginId", router.ownerPluginId("s.cad.cad"), "cad");
check("ownedSurfaceGaps: empty (both roles present)", router.ownedSurfaceGaps(), []);
//#endregion

//#region 🧯️AppRouter faults
checkThrows(
  "surface.conflict on duplicate AppRef",
  () =>
    AppRouter.build([
      { pluginId: "cad", apps: [{ id: surfaceAppId(cad, "editor"), role: "editor", dialect: cad }, { id: surfaceAppId(cad, "editor"), role: "editor", dialect: cad }] },
    ]),
  SURFACE_FAULT_CODES.Conflict,
);
checkThrows(
  "surface.contribution-not-permitted without dependency",
  () =>
    AppRouter.build([
      { pluginId: "cad", artifactKinds: [{ id: "s.cad.cad" }], apps: [{ id: surfaceAppId(cad, "editor"), role: "editor", dialect: cad }] },
      { pluginId: "rogue", apps: [{ id: surfaceAppId(cad, "viewer"), role: "viewer", dialect: cad }] },
    ]),
  SURFACE_FAULT_CODES.ContributionNotPermitted,
);
const incompleteRouter = AppRouter.build([{ pluginId: "cad", artifactKinds: [{ id: "s.cad.cad" }], apps: [{ id: surfaceAppId(cad, "editor"), role: "editor", dialect: cad }] }]);
const gaps = incompleteRouter.ownedSurfaceGaps();
check("surface.missing-owner-surface reported for the missing role only", gaps.length === 1 && gaps[0]?.code === SURFACE_FAULT_CODES.MissingOwnerSurface && gaps[0]?.origin === "framework", true);
checkThrows("surface.unknown-dialect on resolve with no entries", () => resolveOpeningApp(AppRouter.build([]), cad, "editor", EMPTY_OPENING_PREFERENCES), SURFACE_FAULT_CODES.UnknownDialect);
//#endregion

//#region 🧭️OpeningResolver four-step precedence
check("step 2: owner wins with no pin", resolveOpeningApp(router, cad, "editor", EMPTY_OPENING_PREFERENCES), { pluginId: "cad", appId: surfaceAppId(cad, "editor") });
const pinned = { defaults: [{ dialect: cad, role: "editor" as const, app: { pluginId: "aec-building", appId: surfaceAppId(cad, "editor") } }] };
check("step 1: pin wins when still present", resolveOpeningApp(router, cad, "editor", pinned), { pluginId: "aec-building", appId: surfaceAppId(cad, "editor") });
const stalePin = { defaults: [{ dialect: cad, role: "editor" as const, app: { pluginId: "ghost", appId: "ghost#editor" } }] };
check("step 1 skipped when pin no longer in router -> step 2 owner", resolveOpeningApp(router, cad, "editor", stalePin), { pluginId: "cad", appId: surfaceAppId(cad, "editor") });
const noOwnerRouter = AppRouter.build([{ pluginId: "aec-building", apps: [{ id: surfaceAppId(cad, "editor"), role: "editor", dialect: cad }] }]);
check("step 3: first entry when no owner known", resolveOpeningApp(noOwnerRouter, cad, "editor", EMPTY_OPENING_PREFERENCES), { pluginId: "aec-building", appId: surfaceAppId(cad, "editor") });
//#endregion

//#region 🧮️OpeningPreferences fold (event-sourced, never a mutable map)
const setOp = decodeOpeningConfigMutation({ mutation: "setDefaultApp", dialect: cad, role: "editor", app: { pluginId: "aec-building", appId: surfaceAppId(cad, "editor") } });
const clearOp = decodeOpeningConfigMutation({ mutation: "clearDefaultApp", dialect: cad, role: "editor" });
check("decodeOpeningConfigMutation setDefaultApp", setOp, { mutation: "setDefaultApp", dialect: cad, role: "editor", app: { pluginId: "aec-building", appId: surfaceAppId(cad, "editor") } });
const afterSet = foldOpeningPreferences([setOp!]);
check("fold set", afterSet, { defaults: [{ dialect: cad, role: "editor", app: { pluginId: "aec-building", appId: surfaceAppId(cad, "editor") } }] });
const afterClear = foldOpeningPreferences([setOp!, clearOp!]);
check("fold set-then-clear returns to empty", afterClear, EMPTY_OPENING_PREFERENCES);
// 🧮️ Never a mutable map: folding the SAME op list twice from the same base must be referentially
// independent (no shared mutable state leaking between calls).
const first = foldOpeningPreferences([setOp!]);
const second = foldOpeningPreferences([setOp!]);
check("fold is pure — repeated calls don't share mutable state", first === second, false);
check("fold is pure — but structurally equal", first, second);
check("decodeOpeningPreferences whole-record", decodeOpeningPreferences({ defaults: [{ dialect: cad, role: "viewer", app: { pluginId: "cad", appId: surfaceAppId(cad, "viewer") } }] }), {
  defaults: [{ dialect: cad, role: "viewer", app: { pluginId: "cad", appId: surfaceAppId(cad, "viewer") } }],
});
//#endregion

if (failures > 0) {
  console.error(`\n${failures} check(s) FAILED`);
  process.exit(1);
}
console.log("\nAll checks passed");
