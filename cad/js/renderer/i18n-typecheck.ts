/** @emoji 🪁 Ensures CAD play toolbar categories resolve to domain-neutral i18n keys. */
import type { UiTranslationKey } from "@ui/react/i18n-types";

const _cadPlayToolbarKeys = [
  "ui.toolbar.parent.view",
  "ui.toolbar.parent.save",
  "ui.toolbar.parent.transform",
  "ui.toolbar.parent.transfer",
] as const satisfies readonly UiTranslationKey[];
