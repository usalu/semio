"""Ticket input script: the course chrome texts leave the entry chunk, one chunk per language (idempotent record of the edit: it refuses when an anchor is gone).

Applies, in one go so that no peer's build sees half of it:
  - ui i18n: `registerUiTranslationBundle` (one locale) beside `registerUiTranslationBundles`
  - course i18n: bundles imported on demand (`loadCourseLocales`, `courseLocaleLoaded`)
  - course app: the client waits for the texts of its language (every offered one while none is chosen), `mountCourse` renders once they are here
  - course-react tests: every language preloaded for the suites (setup file), a law that each language arrives on its own
"""
import os
import sys

ROOT = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", "..", "..", "..", "..", "..", "..", ".."))
FW = os.path.join(ROOT, "🧰️framework")
REACT = os.path.join(FW, "🛍️products", "📚️course", "🎯️targets", "⚛️react")
PRODUCT = os.path.join(FW, "🛍️products", "📚️course")
files = {}


def load(path):
    if path not in files:
        files[path] = open(path, encoding="utf8").read()
    return files[path]


def edit(path, pairs):
    s = load(path)
    for old, new in pairs:
        if s.count(old) != 1:
            sys.exit(f"anchor missing or not unique in {path}: {old[:90]!r} ({s.count(old)})")
        s = s.replace(old, new, 1)
    files[path] = s


UI = os.path.join(FW, "🔨️modules", "🖱️ui", "🌐️i18n", "🟦️.ts")
edit(UI, [
    ("const registeredUiTranslationBundles: { readonly [L in UiLocale]: { readonly translation: Record<string, unknown> } }[] = [uiChromeTranslationBundles];",
     "const registeredUiTranslationBundles: Partial<{ readonly [L in UiLocale]: { readonly translation: Record<string, unknown> } }>[] = [uiChromeTranslationBundles];"),
    ("function applyUiTranslationBundleTo(instance: typeof i18next, bundle: { readonly [L in UiLocale]: { readonly translation: Record<string, unknown> } }): void {\n  for (const [language, resource] of Object.entries(bundle)) {\n    instance.addResourceBundle(language, \"translation\", resource.translation, true, true);\n  }\n}",
     "function applyUiTranslationBundleTo(instance: typeof i18next, bundle: Partial<{ readonly [L in UiLocale]: { readonly translation: Record<string, unknown> } }>): void {\n  for (const [language, resource] of Object.entries(bundle)) {\n    if (resource !== undefined) instance.addResourceBundle(language, \"translation\", resource.translation, true, true);\n  }\n}"),
    ("// #region 🐚️ShellI18n\n/** 🐚️ Every bundle ever registered",
     "/** 🪁️ Adds the bundle of one locale to the shared UI i18n instance and to every live shell instance — for a product that\n * fetches its languages one at a time, each as a chunk of its own, and registers a language once its texts have arrived. */\nexport function registerUiTranslationBundle(locale: UiLocale, translation: Record<string, unknown>): void {\n  const bundle = { [locale]: { translation } };\n  applyUiTranslationBundleTo(i18next, bundle);\n  registeredUiTranslationBundles.push(bundle);\n  for (const instance of liveShellI18nInstances) applyUiTranslationBundleTo(instance, bundle);\n}\n\n// #region 🐚️ShellI18n\n/** 🐚️ Every bundle ever registered"),
])

I18N = os.path.join(REACT, "🔨️modules", "🌐️i18n", "🟦️.ts")
edit(I18N, [
    ("import { registerUiTranslationBundles, resolveUiLabel, setUiLocale, uiI18n, type DeepUiTranslationKeys } from \"@semio-tech/ui-react/i18n\";\nimport { COURSE_BUNDLE_EN } from \"./🔠️english-bundle/🟦️.ts\";\nimport { COURSE_BUNDLE_DE } from \"./🔡️german-bundle/🟦️.ts\";\n",
     "import { registerUiTranslationBundle, resolveUiLabel, setUiLocale, uiI18n, type DeepUiTranslationKeys, type UiRegisteredTranslationKey } from \"@semio-tech/ui-react/i18n\";\n"),
    ("/** 🔑️ Every key of the quiz chrome as a dot path, e.g. `course.run.submit`. */\nexport type CourseLabelKey = DeepUiTranslationKeys<typeof COURSE_BUNDLE_EN>;\n\nconst courseLabel = registerUiTranslationBundles({ en: { translation: COURSE_BUNDLE_EN }, de: { translation: COURSE_BUNDLE_DE } });\n",
     """/** 📚️ The shape of the chrome of every language: the English bundle's, which the other languages must repeat key for key. */
type CourseBundle = typeof import("./🔠️english-bundle/🟦️.ts").COURSE_BUNDLE_EN;

/** 🔑️ Every key of the quiz chrome as a dot path, e.g. `course.run.submit`. */
export type CourseLabelKey = DeepUiTranslationKeys<CourseBundle>;

const courseLabel = (key: CourseLabelKey): UiRegisteredTranslationKey => key as UiRegisteredTranslationKey;

const bundleLoaders: { readonly [L in CourseLocale]: () => Promise<CourseBundle> } = {
  en: () => import("./🔠️english-bundle/🟦️.ts").then((module) => module.COURSE_BUNDLE_EN),
  de: () => import("./🔡️german-bundle/🟦️.ts").then((module) => module.COURSE_BUNDLE_DE),
};
const loadedLocales = new Set<CourseLocale>();
const loadingLocales = new Map<CourseLocale, Promise<void>>();

function fetchLocale(locale: CourseLocale): Promise<void> {
  if (loadedLocales.has(locale)) return Promise.resolve();
  let pending = loadingLocales.get(locale);
  if (pending === undefined) {
    pending = bundleLoaders[locale]()
      .then((bundle) => {
        registerUiTranslationBundle(locale, bundle);
        loadedLocales.add(locale);
      })
      .finally(() => loadingLocales.delete(locale));
    loadingLocales.set(locale, pending);
  }
  return pending;
}

/** 📥️ Whether the chrome texts of `locale` have arrived: until then nothing of the client resolves in it. */
export function courseLocaleLoaded(locale: CourseLocale): boolean {
  return loadedLocales.has(locale);
}

/** 📥️ Fetches the chrome texts of `locales` — each language is a chunk of its own, so a learner downloads only the
 * language the learner speaks — and settles once they are registered on the shared i18n port; every offered language
 * by default, which is what the language question needs while none is chosen. */
export function loadCourseLocales(locales: readonly CourseLocale[] = COURSE_LOCALES): Promise<void> {
  return Promise.all(locales.map(fetchLocale)).then(() => undefined);
}
"""),
    ("/** 🌐️ Every string the course client itself shows, in English and German, registered on the repo's shared i18n port.\n *",
     "/** 🌐️ Every string the course client itself shows, in English and German, registered on the repo's shared i18n port — each\n * language a chunk of its own that {@link loadCourseLocales} fetches, the chosen language alone or, while none is chosen, all.\n *"),
])

APP = os.path.join(REACT, "🔨️modules", "📚️app", "🟦️.tsx")
edit(APP, [
    ('import { createRoot } from "react-dom/client";', 'import { createRoot, type Root } from "react-dom/client";'),
    ('import { applyLocale, localized, preferredLocale, courseText, titled, type CourseLabelKey, type CourseLocale, type CourseText } from "../🌐️i18n/🟦️.ts";',
     'import { COURSE_LOCALES, applyLocale, courseLocaleLoaded, loadCourseLocales, localized, preferredLocale, courseText, titled, type CourseLabelKey, type CourseLocale, type CourseText } from "../🌐️i18n/🟦️.ts";'),
    ("""/** ❓️ The whole course client for one proctor tenant.""",
     """/** 🌐️ The language the client shows in, once its texts have arrived: `wanted` — every offered language while none is
 * chosen — is fetched as chunks of its own; until they are here the language shown stays the one before, and at the very start
 * nothing is ready. */
function useCourseLocale(wanted: CourseLocale | undefined): { readonly ready: boolean; readonly locale: CourseLocale | undefined } {
  const needed = wanted === undefined ? COURSE_LOCALES : [wanted];
  const arrived = needed.every(courseLocaleLoaded);
  const [, rerender] = useState(0);
  const shown = useRef<{ readonly locale: CourseLocale | undefined } | undefined>(undefined);
  if (arrived) shown.current = { locale: wanted };
  useEffect(() => {
    if (arrived) return;
    let live = true;
    void loadCourseLocales(needed).then(() => live && rerender((count) => count + 1));
    return () => {
      live = false;
    };
  }, [arrived, wanted]);
  return shown.current === undefined ? { ready: false, locale: undefined } : { ready: true, locale: shown.current.locale };
}

/** 🌐️ The language of another learner's screen, once its texts have arrived; the client's own language until then. */
function useLoadedLocale(wanted: CourseLocale, fallback: CourseLocale): CourseLocale {
  const arrived = courseLocaleLoaded(wanted);
  const [, rerender] = useState(0);
  useEffect(() => {
    if (arrived) return;
    let live = true;
    void loadCourseLocales([wanted]).then(() => live && rerender((count) => count + 1));
    return () => {
      live = false;
    };
  }, [arrived, wanted]);
  return arrived ? wanted : fallback;
}

/** ❓️ The whole course client for one proctor tenant."""),
    ("  const locale = preferences.locale ?? preferredLocale(options.languages ?? browserLanguages());\n  const mobile = useMediaQuery(UI_MOBILE_MEDIA_QUERY);",
     "  const wanted = preferences.locale ?? preferredLocale(options.languages ?? browserLanguages());\n  const { ready, locale } = useCourseLocale(wanted);\n  const mobile = useMediaQuery(UI_MOBILE_MEDIA_QUERY);"),
    ("""  useLayoutEffect(() => {
    void applyLocale(locale);
  }, [locale]);
  const title = documentTitle(state, locale);
  useEffect(() => {
    if (locale === undefined) document.title = title;
  }, [locale, title]);""",
     """  useLayoutEffect(() => {
    void applyLocale(locale ?? wanted);
  }, [locale, wanted]);
  const title = ready ? documentTitle(state, locale) : "";
  useEffect(() => {
    if (ready && locale === undefined) document.title = title;
  }, [ready, locale, title]);"""),
    ("  if (locale !== undefined)\n    return (\n      <CoursePetsProvider", "  if (!ready) return <></>;\n  if (locale !== undefined)\n    return (\n      <CoursePetsProvider"),
    ("  const remoteLocale = remote?.view?.locale ?? locale;\n", "  const remoteLocale = useLoadedLocale(remote?.view?.locale ?? locale, locale);\n"),
    ("""  void applyLocale(preferences.locale ?? preferredLocale(options.languages ?? browserLanguages()));
  const reactRoot = createRoot(root);
  reactRoot.render(
    <StrictMode>
      <CourseApp {...options} />
    </StrictMode>,
  );
  return () => reactRoot.unmount();""",
     """  const wanted = preferences.locale ?? preferredLocale(options.languages ?? browserLanguages());
  void applyLocale(wanted);
  let reactRoot: Root | undefined;
  let live = true;
  const render = (): void => {
    if (!live) return;
    reactRoot = createRoot(root);
    reactRoot.render(
      <StrictMode>
        <CourseApp {...options} />
      </StrictMode>,
    );
  };
  const needed = wanted === undefined ? COURSE_LOCALES : [wanted];
  if (needed.every(courseLocaleLoaded)) render();
  else void loadCourseLocales(needed).then(render, render);
  return () => {
    live = false;
    reactRoot?.unmount();
  };"""),
    ("/** 🚀️ Mounts the course client into `root` — with the stored theme and the known language applied to the document\n * before the first paint — and returns the function that unmounts it again. */",
     "/** 🚀️ Mounts the course client into `root` — with the stored theme and the known language applied to the document\n * before the first paint, the page's own loading text standing until the texts of that language (every offered one while\n * none is known) have arrived — and returns the function that unmounts it again. */"),
])

INDEX = os.path.join(REACT, "🟦️.tsx")
edit(INDEX, [("export { COURSE_LOCALES, REJECTION_LABELS, TASK_KIND_LABELS, applyLocale, isCourseLocale, localized, preferredLocale, courseText, titled } from",
              "export { COURSE_LOCALES, REJECTION_LABELS, TASK_KIND_LABELS, applyLocale, courseLocaleLoaded, isCourseLocale, loadCourseLocales, localized, preferredLocale, courseText, titled } from")])

CONFIG = os.path.join(REACT, "🧪️tests", "🎚️config", "🟦️.ts")
edit(CONFIG, [('    setupFiles: [resolve(repoRoot, "🧰️framework/🔨️modules/🖱️ui/🧪️tests/🧹️react-environment/🟦️.ts")],',
               '    setupFiles: [resolve(repoRoot, "🧰️framework/🔨️modules/🖱️ui/🧪️tests/🧹️react-environment/🟦️.ts"), resolve(product, "🧪️tests/🌐️preloaded-locales/🟦️.ts")],')])

SETUP_DIR = os.path.join(PRODUCT, "🧪️tests", "🌐️preloaded-locales")
SETUP = os.path.join(SETUP_DIR, "🟦️.ts")
files[SETUP] = """/** 🌐️ Every language of the client has arrived before a suite starts: the client fetches its texts as chunks of their own
 * and waits for them, which the suites that drive it in either language take as given — the one that holds the chunks to
 * their own laws starts from a fresh module graph.
 * @see ../🗣️translation-completeness/🟦️.tsx — each language arrives on its own
 * @see ../../🎯️targets/⚛️react/🔨️modules/🌐️i18n/🟦️.ts — the loader */
import { loadCourseLocales } from "../../🎯️targets/⚛️react/🔨️modules/🌐️i18n/🟦️.ts";

await loadCourseLocales();
"""

TC = os.path.join(PRODUCT, "🧪️tests", "🗣️translation-completeness", "🟦️.tsx")
edit(TC, [
    ('import { describe, expect, it } from "vitest";', 'import { describe, expect, it, vi } from "vitest";'),
])
s = load(TC)
marker = 'describe("🗣️ translation completeness", () => {\n'
if s.count(marker) != 1:
    sys.exit("translation completeness describe not found")
files[TC] = s.replace(marker, marker + """  it("fetches each language as a chunk of its own: nothing resolves before it has arrived, and one language never brings the other", async () => {
    vi.resetModules();
    const fresh = await import("../../🎯️targets/⚛️react/🔨️modules/🌐️i18n/🟦️.ts");
    const key = "course.app.loading" as const;
    expect([fresh.courseLocaleLoaded("en"), fresh.courseLocaleLoaded("de")]).toEqual([false, false]);
    expect([fresh.courseText("en")(key), fresh.courseText("de")(key)]).toEqual([key, key]);
    await fresh.loadCourseLocales(["en"]);
    expect([fresh.courseLocaleLoaded("en"), fresh.courseLocaleLoaded("de")]).toEqual([true, false]);
    expect([fresh.courseText("en")(key), fresh.courseText("de")(key)]).toEqual(["Loading…", key]);
    await fresh.loadCourseLocales();
    expect([fresh.courseLocaleLoaded("en"), fresh.courseLocaleLoaded("de")]).toEqual([true, true]);
    expect([fresh.courseText("en")(key), fresh.courseText("de")(key)]).toEqual(["Loading…", "Wird geladen…"]);
    expect(await Promise.all([fresh.loadCourseLocales(["de"]), fresh.loadCourseLocales(["de"])])).toEqual([undefined, undefined]);
  });

""", 1)

for path, text in files.items():
    os.makedirs(os.path.dirname(path), exist_ok=True)
    open(path, "w", encoding="utf8").write(text)
print("applied", len(files), "files")
