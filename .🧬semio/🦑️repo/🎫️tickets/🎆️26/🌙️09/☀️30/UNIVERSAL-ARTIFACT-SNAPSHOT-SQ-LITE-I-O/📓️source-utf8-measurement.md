# Bounded Source Unicode Measurement

The registered Run source gate executed 40 laws: 38 passed and the two new long-Unicode interior cancellation laws failed (`root-run-utf8-red.log`). Each law obtains UTF-8 length independently with Bun Buffer, then requires a known UTF-16 traversal frontier in projection or reconstruction. The existing implementation measured the entire TEXT synchronously and offered no interior frontier.

The shared artifact projection and table admission now measure long Unicode text in bounded 16,384-code-unit steps, validate surrogate pairs without an encoded allocation, check cumulative value budgets before and during traversal, and publish progress under the caller's phase. Scalar widths and short text retain existing strict validation. Existing projection sorting remains unchanged. Verification of the repair is pending a fresh registered gate.

This closes measured TEXT traversal only; it does not establish all-artifact completion or prove bounded copying of arbitrary large byte cells.


Fresh registered gate `root-run-curation-source-utf8.log` finished with exit 0: Run 40/40 laws (including both interior Unicode cancellation laws), 101 assertions, 1.68s; Curation 46/46 laws, 87 assertions, 3.85s. Both strict public source checks passed in the same Nx run. The predecessor measured RED was 38 passed and two failed. Native ownership remains independently pending.
