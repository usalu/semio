# Completed WASI Check Cache Reclaimed

The native1134 check encountered ENOSPC after compiler diagnostics had already identified oversized enums. Removed only this ticket's completed wasm32-wasip2/debug compilation cache after confirming wasm1127 has a final receipt and no current command references that cache path. Removed 1713 generated files totaling 3525503784 logical bytes. Native build outputs, all source inputs, exact runtime receipts and diagnostic logs are preserved. The next WASI check will regenerate its target metadata.
