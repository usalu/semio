import sys, pathlib
sys.path.insert(0, str(pathlib.Path(__file__).parent))
from mk import edit, create
H = "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/"
D = "♻️mit-bestand/🧺️demonstrator/"

create(H + "🌐️locale-request/🟦️.ts", '''/** 🌐️ The language a hosting page asks of a shell that is already running, as an ephemeral local-only preference: the page's choice reaches the running shell (and the plugin guests behind it) without remounting it. */

/** 🔁️ The requested locale to commit now: `requested` when it is new to the shell (`applied` is the last one it was given), the shell is not locked to a language, and the host actually asked; otherwise `undefined`. */
export function localeRequestToApply<L extends string>(applied: L, requested: L | undefined, locked: L | undefined): L | undefined {
  return requested === undefined || requested === applied || locked !== undefined ? undefined : requested;
}
''', "web-shellhost-0.patch")

create(H + "🌐️locale-request/🧪️tests/🟦️.ts", '''import { expect, test } from "bun:test";
import { localeRequestToApply } from "../🟦️.ts";

test("a new request is applied once", () => {
  expect(localeRequestToApply("en", "de", undefined)).toBe("de");
  expect(localeRequestToApply("de", "de", undefined)).toBeUndefined();
});

test("no request, or a locked language, applies nothing", () => {
  expect(localeRequestToApply("en", undefined, undefined)).toBeUndefined();
  expect(localeRequestToApply("en", "de", "en")).toBeUndefined();
});
''', "web-shellhost-1.patch")

edit(H + "🟦️.tsx", [
 ('import { useCallback, useEffect, useLayoutEffect, useState } from "../⚛️react-port/🟦️.ts";\n', 'import { useCallback, useEffect, useLayoutEffect, useRef, useState } from "../⚛️react-port/🟦️.ts";\nimport { localeRequestToApply } from "./🌐️locale-request/🟦️.ts";\n'),
 ("<FrameworkOsShellMounted {...props} initialLocale={initialLocale} />;", "<FrameworkOsShellMounted {...props} initialLocale={initialLocale} requestedLocale={requestedLocale} />;"),
 ("function FrameworkOsShellMounted(props: FrameworkOsShellProps & { readonly initialLocale: ShellLocale }): React.ReactElement {\n  const { shellId, storageNamespace, ownsPage = false, brand, locks, initialLocale, ...innerProps } = props;",
  "function FrameworkOsShellMounted(props: FrameworkOsShellProps & { readonly initialLocale: ShellLocale; readonly requestedLocale: ShellLocale | undefined }): React.ReactElement {\n  const { shellId, storageNamespace, ownsPage = false, brand, locks, initialLocale, requestedLocale, ...innerProps } = props;"),
 ("  useEffect(() => () => disposeShellI18nInstance(scope.i18n), [scope]);\n",
  "  useEffect(() => () => disposeShellI18nInstance(scope.i18n), [scope]);\n  const appliedLocale = useRef<ShellLocale>(requestedLocale ?? initialLocale);\n  useEffect(() => {\n    const next = localeRequestToApply(appliedLocale.current, requestedLocale, locks?.locale);\n    if (next === undefined) return;\n    appliedLocale.current = next;\n    commitUiPreferencesConfigMutation(scope.storage, setLocale(next));\n  }, [requestedLocale, locks?.locale, scope]);\n"),
], "web-shellhost-2.patch")

edit(D + "🟦️.tsx", [
 ("  const locks = useMemo(() => resolveShellLocks(pane.brand.locks), [pane.brand]);", "  const locks = useMemo(() => resolveShellLocks(pane.brand.locks), [pane.brand.locks]);"),
 ("      <LayeredOverview\n        key={locale}\n", "      <LayeredOverview\n"),
], "web-demonstrator-1.patch")

edit(D + "🧪️tests/🎭️acceptance/🟦️.ts", [
 ("//#endregion 🃏️OverviewCards\n", '''
/** 🔤️ Switching the language leaves an opened pane running: the very shell element (marked once it is ready) survives both switches and its root follows the language. */
test("language switch keeps panes", async ({ page }) => {
  test.setTimeout(TEST_TIMEOUT_MS);
  await page.goto("/", { waitUntil: "domcontentloaded" });
  await dismissIntroduction(page, page.locator('[id="ui.introduction.skip"]'));
  const paneId = "koordinator";
  await page.locator(`[data-layered-card="${paneId}"] [data-slot="demonstrator-pane-card-open-chip"]`).click();
  expect(await waitForPaneShellOutcome(page, paneId)).toBe("ready");
  await dismissIntroductionIfPresent(page, paneId);
  const shell = page.locator(`[data-shell-id="${paneId}"]`);
  await shell.evaluate((element) => element.setAttribute("data-remount-probe", "kept"));
  await page.locator("[data-layered-overview-button]").click();
  const switcher = page.locator('[data-slot="demonstrator-locale-switcher"]');
  for (const { language } of [LANGUAGE_CASES[1], LANGUAGE_CASES[0]]) {
    await switcher.locator(`button[data-locale="${language}"]`).click();
    await expect(page.locator("html")).toHaveAttribute("lang", language);
    await expect(shell).toHaveAttribute("lang", language);
    await expect(shell).toHaveAttribute("data-remount-probe", "kept");
  }
});
//#endregion 🃏️OverviewCards
'''),
], "web-acceptance-1.patch")
