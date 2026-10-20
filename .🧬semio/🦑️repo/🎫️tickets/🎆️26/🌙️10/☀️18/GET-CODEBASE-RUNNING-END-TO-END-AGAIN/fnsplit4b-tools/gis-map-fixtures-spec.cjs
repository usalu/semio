const { R, f, ph, write } = require("./lib_ps.cjs");
const file = R + "🌎️hub/🧪️tests/🌍️gis-map-fixtures/🟦️.ts";
write({
  file, function: "proveGisMapProposalApprovalFixture", keep: [260, 261, 262], resolve: true, typeOverrides: { fixture: "any", ledger: "any" },
  files: [
    f("🗳", "proposal-fixture-contract", "The fixture schema, committer law and proposal geometry checks of the GIS Map proposal approval fixture proof.", [
      ph("checkProposalFixtureSchema", "🧾", "Loads the proposal approval fixture and checks its schema, binding, limits, preview and lifecycle kinds.", 263, 275),
      ph("checkCommitterLawTables", "⚖", "Checks the scalar and ordered committer law tables and the fixture row counts.", 276, 340),
      ph("checkProposalHashAndGeometry", "📐", "Checks the proposal and inverse hashes, the created region, the base bounds and the lifecycle transitions.", 342, 396),
    ]),
    f("🛡", "visibility-and-committer-sources", "The visibility, error status, committer ledger and committer source checks of the GIS Map proposal approval fixture proof.", [
      ph("checkVisibilityErrorsAndCommitterLedger", "👁", "Checks the role visibility rows, the error statuses and the committer ledger rows.", 404, 416),
      ph("checkRetainedRuntimeSources", "🧬", "Checks the retained GIS Map runtime, binding, wal and sqlite sources.", 471, 517),
    ]),
    f("🚪", "ingress-and-process-sources", "The hub ingress, frozen binary and process source checks of the GIS Map proposal approval fixture proof.", [
      ph("checkHubIngressSources", "🛰", "Checks the hub approval ingress, handler, teardown and registration sources.", 534, 568),
      ph("checkProcessSources", "🏃", "Checks the compiled process runner source and its hostile mutations.", 575, 676),
    ]),
  ],
}, process.argv[2]);
