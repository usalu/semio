const { R, f, ph, write, dir, doc } = require("./lib_ps.cjs");
const file = R + "🌎️hub/🧪️tests/🎫️invite-redemption-checks/🟦️.ts";
write({
  file, function: "proveInviteRedemptionTransaction", keep: [381, 383], resolve: true, typeOverrides: { client: "any", acceptanceMarker: "(value: unknown) => boolean" }, shared: { dir: dir("📐", "invite-redemption-fixture-type"), doc: doc("📐", "The shared invite redemption fixture type.") },
  files: [
    f("🎟", "redemption-fixture-and-oracle", "The fixture contract, sqlite oracle and hostile call checks of the invite redemption transaction proof.", [
      ph("checkRedemptionFixtureContract", "📜", "Loads the invite redemption fixture and checks its schema, limits, vector names and acceptance marker.", 58, 75),
      ph("checkRedemptionOracleVectors", "🔮", "Replays every redemption vector against the sqlite oracle.", 77, 131),
      ph("checkRedemptionCallsAndHostiles", "☠", "Checks every redemption call against the call schema and every hostile mutation against the fixture.", 219, 221),
    ]),
    f("🔒", "backend-source-closure", "The production source closure checks of the invite redemption transaction proof.", [
      ph("checkRedemptionSourceClosure", "🧱", "Reads the directory backend sources and checks the redemption transaction closure with its hostile mutations.", 238, 296),
      ph("checkStateAuthorityClosure", "🏛", "Checks the invite state and ingress authority closure with its hostile mutations.", 300, 340),
      ph("checkArchiveClosure", "🗄", "Checks the archive transaction and rollback closure with its hostile mutations.", 343, 380),
    ]),
  ],
}, process.argv[2]);
