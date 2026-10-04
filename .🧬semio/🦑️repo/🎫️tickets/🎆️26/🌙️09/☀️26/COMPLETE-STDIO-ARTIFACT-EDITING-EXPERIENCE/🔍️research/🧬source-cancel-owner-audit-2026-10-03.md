# Source Cancellation Owner Audit

The new Source Apply lifecycle exposes exact decimal-u64 operation and generation values, converts them to the fixed 16-hex native cancellation authority, and retains the edited draft while cancellation is pending. The execution lane reports a 46/46 focused green and a green aggregate renderer typecheck. Native/browser cancellation acceptance remains open.

The root read-only review found a further required authority check: ShellHost currently creates the cancellation closure through `onAction`, which resolves the current window/program/document. Exact operation and generation alone do not prove the same plugin instance is still the owner if a window/document is replaced. The admitting owner/plugin/instance must remain authoritative, or the handle must be refused before rerouting. A late `lifecycle.started` must also respect retirement. The execution lane has been asked to implement and test these owner replacement cases.

A second required race law distinguishes accepted cancellation from a native terminal/no-op answer. Successful publication that wins the race must still reconcile through its real receipt; a no-op cancellation must not make the browser discard the original outcome. The execution lane owns this native terminal check.
