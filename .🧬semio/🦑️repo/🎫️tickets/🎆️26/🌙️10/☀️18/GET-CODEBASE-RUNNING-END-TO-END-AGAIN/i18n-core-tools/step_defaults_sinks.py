"""Part of increment 2: a sink or a boundary outside the CLI door never crashes for want of a language; when none is chosen it speaks every language."""
from lib import *

PG = LANG + "/📣️progress/🟦️.ts"
PT = LANG + "/📣️progress/🧪️tests/🟦️.ts"
ET = LANG + "/🚨️error/🧪️tests/🟦️.ts"

OLD_SINK = """    const language = resolveLanguageV1(CATALOG_LANGUAGES_V1, options.args ?? [], options.environment ?? process.env, options.osLanguages).language;
    write(event.level === "info" ? "stdout" : "stderr", renderProgressV1(event, language));"""
NEW_SINK = """    const languages = resolveLanguagesV1(CATALOG_LANGUAGES_V1, options.args ?? [], options.environment ?? process.env, options.osLanguages);
    write(event.level === "info" ? "stdout" : "stderr", languages.map((language) => renderProgressV1(event, language)).join("\\n"));"""
OLD_REPORT = """  const language = resolveLanguageV1(CATALOG_LANGUAGES_V1, options.args ?? [], options.environment ?? process.env, options.osLanguages).language;
  (options.write ?? processWriteV1)("stderr", renderRepoErrorV1(error, language));"""
NEW_REPORT = """  const languages = resolveLanguagesV1(CATALOG_LANGUAGES_V1, options.args ?? [], options.environment ?? process.env, options.osLanguages);
  (options.write ?? processWriteV1)("stderr", languages.map((language) => renderRepoErrorV1(error, language)).join("\\n"));"""
OLD_THROW = """  expect(() => createCliSinkV1({ environment: {}, osLanguages: () => [], write: () => undefined })(event)).toThrow(/--lang=en\\|de[\\s\\S]*--lang=en\\|de/u);"""
NEW_THROW = """  const spoken: string[] = [];
  createCliSinkV1({ environment: {}, osLanguages: () => [], write: (_stream, line) => spoken.push(line) })(event);
  expect(spoken).toEqual(["[steps] done.\\n[steps] fertig."]);"""
NEW_ERROR_TEST = """test("a refusal without a chosen language is printed in every language and a wrong choice is refused", () => {
  const lines: string[] = [];
  reportRepoErrorV1(repoErrorV1(faults, "stale", "stale"), { environment: {}, osLanguages: () => [], write: (_stream, line) => lines.push(line), setExitCode: () => undefined });
  expect(lines).toEqual(["The receipt is stale.\\nDie Quittung ist veraltet."]);
  expect(() => reportRepoErrorV1(repoErrorV1(faults, "stale", "stale"), { environment: { SEMIO_LANGUAGE: "fr" }, osLanguages: () => [], write: () => undefined })).toThrow("nicht angebotene Sprache");
});

"""


def run(tree):
    tree.edit(PG, "import { resolveLanguageV1, type LanguageEnvironmentV1, type OsLanguageListV1 }", "import { resolveLanguagesV1, type LanguageEnvironmentV1, type OsLanguageListV1 }")
    tree.edit(PG, "and refuses bilingually when none is chosen. */", "and speaks every language when none is chosen (the CLI door has refused before any line gets here); a wrong explicit choice is refused. */")
    tree.edit(PG, OLD_SINK, NEW_SINK)
    tree.edit(PG, OLD_REPORT, NEW_REPORT)
    tree.edit(PG, "is rendered in the resolved language to the standard error", "is rendered in the resolved language (in every language when none is chosen) to the standard error")
    tree.edit(PT, OLD_THROW, NEW_THROW)
    tree.edit(PT, "and refuses bilingually when none is chosen", "speaks every language when none is chosen and refuses a wrong choice")
    text = tree.text(ET)
    a = text.index('test("a refusal cannot be rendered without a language')
    b = text.index('test("the refusal class is a real error with a stack"')
    tree.put(ET, text[:a] + NEW_ERROR_TEST + text[b:])
