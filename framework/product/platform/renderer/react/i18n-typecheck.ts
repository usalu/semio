/** @emoji 🪁 Compile-time gate: every {@link AppToolCategory} maps to a domain-neutral toolbar parent i18n key. */
import type { AppToolCategory } from "@framework/core";
import type { AssertUiToolbarParentKeysCovered } from "@ui/react";

const _assertFrameworkToolbarParentKeys: AssertUiToolbarParentKeysCovered<AppToolCategory> = true;
