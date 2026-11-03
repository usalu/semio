import sys

p = sys.argv[1]
s = open(p, encoding="utf8").read()


def rep(a, b):
    global s
    assert a in s, a[:70]
    s = s.replace(a, b, 1)


rep('export type Status = "ok" | "unavailable" | "range-end" | "fail";', 'export type Status = "ok" | "unavailable" | "range-end" | "group-drift" | "fail";')
rep("  readonly rangeEnd?: Readonly<Record<string, Failures>>;\n};", "  readonly rangeEnd?: Readonly<Record<string, Failures>>;\n  readonly groupDrift?: Readonly<Record<string, Failures>>;\n};")
rep('import { derivedFlags } from "./🪞️derived.ts";', 'import { derivedFlags } from "./🪞️derived.ts";\nimport { anchorLandmarks, boneSync } from "./⚓️landmarks.ts";')
rep(
    "  if (!subject.isDefault) return;\n  const size =",
    """  for (const finding of boneSync(state, tolerance)) {
    const [a, b] = finding.pair.split("/") as [Viewpoint, Viewpoint];
    collector.fail([a, b], MODE_LIST, finding.check, subject.id, finding.message, { deltaMm: finding.deltaMm, pair: `${finding.pair} ${finding.bone}` });
  }
  for (const finding of anchorLandmarks(state, tolerance)) collector.fail([finding.view], MODE_LIST, "anchor-landmark", subject.id, finding.message, { deltaMm: finding.deltaMm, pair: `${finding.view} ${finding.anchor}` });
  if (!subject.isDefault) return;
  const size =""",
)
rep(
    "      const failures = rangeEndCheck ? rest : all;",
    """      const groupChecks = Object.keys(rest).filter((check) => check.startsWith("group-drift-"));
      const hard = Object.fromEntries(Object.entries(rangeEndCheck ? rest : all).filter(([check]) => !check.startsWith("group-drift-")));
      const failures = hard;
      const groupDrift = Object.fromEntries(groupChecks.map((check) => [check, rest[check]!]));""",
)
rep(
    ': rangeEndCheck ? "range-end" : reason',
    ': groupChecks.length > 0 ? "group-drift" : rangeEndCheck ? "range-end" : reason',
)
rep(
    "...(rangeEndCheck ? { rangeEnd: { \"range-end-unsuitable\": rangeEnd! } } : {}) };",
    "...(rangeEndCheck ? { rangeEnd: { \"range-end-unsuitable\": rangeEnd! } } : {}), ...(groupChecks.length > 0 ? { groupDrift } : {}) };",
)
rep('"range-end": 0, fail: 0 });', '"range-end": 0, "group-drift": 0, fail: 0 });')
rep('"range-end": number; fail: number };', '"range-end": number; "group-drift": number; fail: number };')
open(p, "w", encoding="utf8").write(s)
