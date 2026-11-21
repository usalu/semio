# Layout Export Job Return

The guest compiler rejected the Layout export engine’s obsolete one-argument `InteractiveJob::step`. The document export and media export wrappers had the same signature.

All three trait methods now receive the original return claim and wrap the existing callback with `InteractiveJobStep::from_callback`. The framework retains the claim and receives the callback’s original outcome or failure. The document and media wrappers reuse the existing borrowed engine callback. The media path still qualifies the original Drawing source before passing its read into the engine. Existing fidelity, large-sheet and dispatch tests advance the same borrowed callback under their funded fixture caller.

The pinned Rust parser accepted the three production files and three test leaves, and the whitespace check passed. The shared guest build will verify the updated integration.
