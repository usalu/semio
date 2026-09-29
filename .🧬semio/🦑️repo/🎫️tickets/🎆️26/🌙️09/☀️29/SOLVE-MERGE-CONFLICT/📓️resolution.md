# Merge Conflict Resolution

## Conflict

Stash pop conflict in `🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🟦️.tsx` between:

- **Updated upstream**: i18n bundles extracted to `./🌐️i18n/🟦️.ts` and re-exported via the I18n Port region.
- **Stashed changes**: ~1900 lines of inline German/English `uiChromeTranslationBundles` definitions still present in the barrel file.

## Resolution

Kept **upstream** (removed stashed inline bundles). Translation data already lives in `🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🌐️i18n/🟦️.ts`; the barrel imports and re-exports `uiChromeTranslationBundles` from there.

## Post-fix structure

```
// #endregion 🔑️Schema & Keys
// #region 🔌️I18n Port
import { ..., uiChromeTranslationBundles, ... } from "./🌐️i18n/🟦️.ts";
export { ..., uiChromeTranslationBundles, ... };
```
