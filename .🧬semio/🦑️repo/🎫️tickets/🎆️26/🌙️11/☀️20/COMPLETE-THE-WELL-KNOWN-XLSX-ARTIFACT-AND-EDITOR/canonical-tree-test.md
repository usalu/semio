# Persistent canonical JSON test

The canonical JSON test crate failed to compile before running the XLSX mutation payload regression. Its persistent-row test passed allocation grants to operations that now require an original allocation effect. It also called removed source projection and sealed-value admission methods.

The affected file is `🧰️framework/🔨️modules/🎒️pack/🔤️json/🛫️encode/🧭️tree/📋️persistent/🧪️tests/🦀️.rs`. The test now drives source admission, cursor movement and retirement through the existing effect bank. It borrows the projection through the same retained alias operation as the production canonical encoder, and supplies each retirement turn's body limit. Sealed rows and text use the original source admission demand. The test still checks row identity, exact heap receipts, complete canonical bytes, cancellation prefixes, and allocation balance against its existing fixture and serde_json oracle.

The first native rerun compiled the test crate, then exposed an owned-source publication requirement. The test follows the existing identity-projection law through the funded `projectionStart` turn and both `ready` publication turns.

The next native run passed the tagged XLSX payload regression and reached the persistent cursor's sealed retirement. The test now lends the same EffectBank source identity through the existing original producer helpers for cursor advancement, source closure, and text closure. The persistent regression still needs a rerun. The patched file passes the repository whitespace check.
