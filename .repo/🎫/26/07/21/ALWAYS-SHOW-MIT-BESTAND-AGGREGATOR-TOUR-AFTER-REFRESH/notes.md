# Always Show Mit Bestand Aggregator Tour After Refresh

## Intent

Aggregator brand introduction must auto-start after every window refresh, not only on first device visit.

## Approach

Brand-level `ShellBrand.replayIntroductionOnLoad`:
- when true, shell ignores `ui.introduction.seen.*` on auto-start
- when true, shell skips writing the seen flag on dismiss/complete
- enabled for `entwerfen-mit-bestand`

## Verify

```bash
bun nx run @semio-tech/framework-renderer-react:test -- -t "shouldReplayIntroductionOnLoad|Introduce App"
bun .repo/🎫/26/07/21/ALWAYS-SHOW-MIT-BESTAND-AGGREGATOR-TOUR-AFTER-REFRESH/verify-policy.ts
```
