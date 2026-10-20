const { R, f, ph, write } = require("../lib_ps.cjs");
const spec = {
  file: R + "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🚀️apply/🟦️.ts",
  function: "applyTaxonomyPlan", keep: [487], allowLets: ["journal"], resolve: true, typesFromProducers: true,
  files: [
    f("📜", "plan-checks", "The plan and transaction name checks of the taxonomy plan apply.", [
      ph("bindTransactionNames", "🏷", "Verifies the plan against the options and binds the canonical transaction, attempt, lease and preparation names.", 38, 78),
      ph("checkPlanAuthority", "🛡", "Captures the plan bytes, checks the plan and cancellation file stay outside the transaction and reads the transaction ancestors.", 79, 126),
    ]),
    f("🔎", "attempt-discovery", "The existing attempt discovery and validation of the taxonomy plan apply.", [
      ph("discoverTransactionAttempts", "🔭", "Reads the existing and unpublished attempt directories of the plan transaction.", 127, 185),
      ph("defineAttemptValidators", "✅", "Validates the unpublished and terminal attempts and closes the terminal ones.", 191, 274),
    ]),
    f("🎯", "attempt-selection", "The attempt selection and lease handling of the taxonomy plan apply.", [
      ph("selectAttempt", "🧭", "Selects the resumed attempt or allocates the next attempt ordinal and binds its journal paths.", 306, 325),
      ph("defineLeaseAndPreChecks", "🔑", "Defines the lease acquisition and release and checks the symlink targets, baseline commit and cancellation.", 326, 351),
    ]),
    f("🛫", "preflight-and-lock", "The preflight reference basis and the resume snapshot lock of the taxonomy plan apply.", [
      ph("capturePreflight", "🧪", "Captures the preflight reference basis and verifies the pre-state of every planned operation.", 352, 352),
      ph("lockResumeSnapshot", "🔒", "Validates the selected resume snapshot and locks the owned lease for it.", 433, 475),
    ]),
    f("🪪", "journal-preparation", "The journal preparation of the taxonomy plan apply.", [
      ph("prepareAttemptJournal", "📒", "Loads the resumed journal or publishes the initial journal and lease of a new attempt.", 493, 627),
    ]),
  ],
};
write(spec, process.argv[2]);
