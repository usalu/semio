// 🔗️ Lane 1-D parity reconciliation (ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET,
// 📓️w1-d-report.md): the SAME ordered fixture as the Rust twin
// (💻️os/🔌️plugin/🖥️host/🦀️component.rs, `app_router_tests::w1_d_parity_fixture_owner_two_contributors_duplicate_and_unknown_dialect`)
// — owner surface, two contributed surfaces from different plugins, a duplicate, an unknown
// dialect — run here through the TS `AppRouter`/`resolveOpeningApp`. Both sides must produce
// identical `entriesFor`/`surfaces_for` ordering and identical fault codes. Run once, ad hoc:
//   bun run 🧪️w1-d-parity.ts
import { AppRouter, resolveOpeningApp, EMPTY_OPENING_PREFERENCES, SURFACE_FAULT_CODES, surfaceAppId, SemioFaultError, type AppRouterManifest, type ArtifactDialect } from "../../../../../../🧰️framework/🔨️modules/🎠️kernel/🟦️component.ts";

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
function checkFaultCode(label: string, fn: () => void, expectedCode: string): void {
  try {
    fn();
    failures += 1;
    console.error(`[FAIL] ${label}: expected throw with code ${expectedCode}, got none`);
  } catch (error) {
    const code = error instanceof SemioFaultError ? error.fault.code : undefined;
    if (code === expectedCode) {
      console.log(`[ok] ${label} (code=${code})`);
    } else {
      failures += 1;
      console.error(`[FAIL] ${label}: wrong code ${code}, expected ${expectedCode}`);
    }
  }
}

const cad: ArtifactDialect = { artifactKind: "s.cad.cad", standard: "1", subset: "*" };

//#region 🧭️Owner surface + two contributed surfaces from different plugins, ordered
const manifests: AppRouterManifest[] = [
  { pluginId: "cad", artifactKinds: [{ id: "s.cad.cad" }], apps: [{ id: surfaceAppId(cad, "editor"), role: "editor", dialect: cad }] },
  { pluginId: "norm", dependencies: [{ pluginId: "cad", version: "*" }], apps: [{ id: "s.cad.cad@1/*#editor-norm", role: "editor", dialect: cad }] },
  { pluginId: "aec-building", dependencies: [{ pluginId: "cad", version: "*" }], apps: [{ id: "s.cad.cad@1/*#editor-aec", role: "editor", dialect: cad }] },
];
const router = AppRouter.build(manifests);
check("owner first, then pluginId ascending (aec-building < norm)", router.entriesFor(cad, "editor"), [
  { pluginId: "cad", appId: surfaceAppId(cad, "editor") },
  { pluginId: "aec-building", appId: "s.cad.cad@1/*#editor-aec" },
  { pluginId: "norm", appId: "s.cad.cad@1/*#editor-norm" },
]);
//#endregion

//#region 🧯️A duplicate AppRef
checkFaultCode(
  "duplicate AppRef -> surface.conflict",
  () =>
    AppRouter.build([
      ...manifests,
      { pluginId: "aec-building", dependencies: [{ pluginId: "cad", version: "*" }], apps: [{ id: "s.cad.cad@1/*#editor-aec", role: "editor", dialect: cad }] },
    ]),
  SURFACE_FAULT_CODES.Conflict,
);
//#endregion

//#region 🧯️An unknown dialect
const unknownDialect: ArtifactDialect = { artifactKind: "s.cad.cad", standard: "1", subset: "does-not-exist" };
checkFaultCode("unknown dialect -> surface.unknown-dialect", () => resolveOpeningApp(router, unknownDialect, "editor", EMPTY_OPENING_PREFERENCES), SURFACE_FAULT_CODES.UnknownDialect);
//#endregion

if (failures > 0) {
  console.error(`\n${failures} check(s) FAILED`);
  process.exit(1);
}
console.log("\nAll checks passed");
