@capability-live-commands
@oracle-apply-remedy-equivalence
@comparison-ghost-equals-applied-write
Feature: Live commands batch field writes and keep previews out of the store
  Executed by `🔬️unit/🦀️.rs`.

  Further specification assertions, asserted by the Rust unit tests of the crate and not by the platform adapters:
  - One bundle is one emit: Given a drag release with two bound fields When setFields resolves the bundle Then one emit carries one mutation per leaf and counts are typed like their leaf
  - A bad write fails the bundle: Given a bundle whose second path does not exist When setFields resolves it Then nothing is emitted
  - A remedy preview equals the applied remedy: Given a failing check with an at-least remedy When the remedy is previewed and then applied to a copy Then the ghost write equals the applied leaf and the document is untouched
  - Preview, clear and progress commands never touch the document: Given the live tool ids previewRemedy, clearPreview, cancelEvaluation and evaluateUnits When their publication contracts are read Then each publishes on the transient lane only
