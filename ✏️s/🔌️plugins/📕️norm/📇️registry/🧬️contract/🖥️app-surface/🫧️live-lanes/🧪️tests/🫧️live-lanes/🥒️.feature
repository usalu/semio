@capability-live-lanes
@oracle-serde-json-canonical
@comparison-canonical-json-v1
Feature: Lane payloads carry provisional state only
  Executed by `🔬️unit/🦀️.rs`; document truth stays in store events.

  Further specification assertions, asserted by the Rust unit tests of the crate and not by the platform adapters:
  - An aborted drag leaves zero trace: Given a default transient When a drag preview is opened and then cleared Then the transient equals the default transient
  - A drag preview resolves handle writes: Given a length handle limited to 12 m When the preview is taken at parameter 99 Then it carries the write "members[id=b1].length" = 12.0
  - Overlay order: Given a drag and a remedy ghost When the overlay is listed Then drag writes come before ghost writes
  - Presence round-trips for peers: Given a presence with selection, camera and drag When it is encoded and decoded Then it is equal
