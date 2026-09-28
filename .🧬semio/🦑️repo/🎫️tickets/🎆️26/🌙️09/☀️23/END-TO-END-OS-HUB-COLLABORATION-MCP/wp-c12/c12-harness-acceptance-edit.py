"""🧾️ C12 session 14b: collab-e2e publishes its goal-gate record (preamble rule 17; R10 ids `react-collaboration-e2e-en|de`) through
`withAcceptanceRecord` + `publishAcceptanceCheckResult`, takes `--hub <url>` and `--locale en|de` (the two browser contexts run in
that language; the dialog option labels follow it). One-off, idempotent."""
import sys

ROOT = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests"
COLLAB = ROOT + "/🤝️collaboration/🟦️.ts"
VERIFY = ROOT + "/✅️verification/🟦️.ts"
text = open(COLLAB, encoding="utf-8").read()
verify = open(VERIFY, encoding="utf-8").read()
if "runCollabE2eCli" in text:
    print("already applied")
    sys.exit(0)


def sub(old: str, new: str) -> None:
    global text
    assert text.count(old) == 1, (text.count(old), old[:100])
    text = text.replace(old, new)


sub('import { decodeClientFrame } from "../../../../../../🔨️modules/📡️replication/🟦️.ts";\n', '''import { decodeClientFrame } from "../../../../../../🔨️modules/📡️replication/🟦️.ts";

import { acceptanceCheckResult, publishAcceptanceCheckResult, withAcceptanceRecord } from "../../../../../🦑️repo/🔨️modules/🧪️test/🎯️acceptance/📋️orchestration/🟦️.ts";
''')

sub('''/** 🧾️ One step's verdict: `true` PASS, `false` FAIL, `null` SKIP (the run does not own what the step needs). */''', '''/** 🌍️ The language both humans' browser contexts run in (`--locale`), and the dialog option labels STEP 1/2 pick in it — the
 * Space app's own `LocalizedLabel`s (`🪐️space/…/🏠️home/…/✏️editor/🦀️.rs`). */
type CollabLocale = "en" | "de";

const COLLAB_E2E_LOCALES: Readonly<Record<CollabLocale, { readonly browser: string; readonly studio: string; readonly public: string; readonly author: string }>> = {
  en: { browser: "en-US", studio: "Studio", public: "Public", author: "Author" },
  de: { browser: "de-DE", studio: "Studio", public: "Öffentlich", author: "Autor" },
};

/** 🧾️ One step's verdict: `true` PASS, `false` FAIL, `null` SKIP (the run does not own what the step needs). */''')

sub('''function collabExternalHub(): CollabExternalHub | null {
  const baseUrl = process.env.S_COLLAB_HUB_URL;
  if (!baseUrl) return null;''', '''function collabExternalHub(baseUrl: string | undefined): CollabExternalHub | null {
  if (!baseUrl) return null;''')

sub('''  adminCapability: () => string,
  user1Sent: CollabSentCommands,
): Promise<{ readonly spaceId: string | undefined; readonly artifactId: string | undefined }> {''', '''  adminCapability: () => string,
  user1Sent: CollabSentCommands,
  locale: CollabLocale,
): Promise<{ readonly spaceId: string | undefined; readonly artifactId: string | undefined }> {''')
sub('''    await collabSelectOption(user1, "kind", "Studio");
    await collabSelectOption(user1, "visibility", "Public");''', '''    await collabSelectOption(user1, "kind", COLLAB_E2E_LOCALES[locale].studio);
    await collabSelectOption(user1, "visibility", COLLAB_E2E_LOCALES[locale].public);''')
sub('''      await collabSelectOption(user1, "role", "Author");''', '''      await collabSelectOption(user1, "role", COLLAB_E2E_LOCALES[locale].author);''')

sub('''async function runCollabE2eVerify(): Promise<void> {
  const outDir = collabOutDir();
  const external = collabExternalHub();''', '''async function runCollabE2eVerify(opts: { readonly hub: string | undefined; readonly locale: CollabLocale; readonly check: string }): Promise<void> {
  const startedAt = new Date();
  const outDir = collabOutDir();
  const external = collabExternalHub(opts.hub);''')
sub('''    const context1 = await browser.newContext({ viewport: { width: 1440, height: 900 } });
    const context2 = await browser.newContext({ viewport: { width: 1440, height: 900 } });''', '''    const context1 = await browser.newContext({ viewport: { width: 1440, height: 900 }, locale: COLLAB_E2E_LOCALES[opts.locale].browser });
    const context2 = await browser.newContext({ viewport: { width: 1440, height: 900 }, locale: COLLAB_E2E_LOCALES[opts.locale].browser });''')
sub('''() => (external ? collabExternalAdminCapability(external) : hubDaemon!.adminCapability), user1Sent);''', '''() => (external ? collabExternalAdminCapability(external) : hubDaemon!.adminCapability), user1Sent, opts.locale);''')
sub('''  for (const outcome of [...results].sort((left, right) => left.step - right.step)) console.log(`  STEP ${outcome.step}: ${collabVerdict(outcome.pass)}: ${outcome.name}`);
  if (passed !== results.length) process.exitCode = 1;
}
''', '''  for (const outcome of [...results].sort((left, right) => left.step - right.step)) console.log(`  STEP ${outcome.step}: ${collabVerdict(outcome.pass)}: ${outcome.name}`);
  const failing = results.filter((outcome) => outcome.pass === false).map((outcome) => outcome.step).sort((left, right) => left - right);
  const status = results.length === COLLAB_E2E_STEP_NAMES.length && failing.length === 0 ? "pass" : "fail";
  publishAcceptanceCheckResult(
    repoRoot,
    acceptanceCheckResult({
      check: opts.check,
      status,
      startedAt,
      measured: { locale: opts.locale, steps: COLLAB_E2E_STEP_NAMES.length, recorded: results.length, passed, skipped, failed: failing.length, routeMisses: collabRouteMisses.length, externalHub: external !== null },
      summary: {
        en: `${passed}/${COLLAB_E2E_STEP_NAMES.length} collaboration steps pass in ${opts.locale} (${skipped} skipped)${failing.length ? `; failing: STEP ${failing.join(", ")}` : ""}`,
        de: `${passed}/${COLLAB_E2E_STEP_NAMES.length} Zusammenarbeitsschritte bestehen in ${opts.locale} (${skipped} übersprungen)${failing.length ? `; fehlgeschlagen: SCHRITT ${failing.join(", ")}` : ""}`,
      },
      evidence: [outDir],
    }),
  );
  if (status !== "pass") process.exitCode = 1;
}

/** 🚪️ `verify collab [--hub <url>] [--locale en|de]` — the two-human React collaboration journey against the hub `--hub` names
 * (`S_COLLAB_HUB_URL` otherwise; neither: this run boots its own hub), both humans in `--locale`; the goal-gate record is
 * `react-collaboration-e2e-<locale>`, `blocked` when the hub cannot be reached. Credentials only via env. */
async function runCollabE2eCli(segments: readonly string[]): Promise<void> {
  const flag = (name: string): string | undefined => {
    const value = segments[segments.indexOf(name) + 1];
    return segments.includes(name) && value !== undefined && !value.startsWith("--") ? value : undefined;
  };
  const locale: CollabLocale = flag("--locale") === "de" ? "de" : "en";
  const check = `react-collaboration-e2e-${locale}`;
  await withAcceptanceRecord(repoRoot, check, () => runCollabE2eVerify({ hub: flag("--hub") ?? process.env.S_COLLAB_HUB_URL, locale, check }), (error) => /ECONNREFUSED|ERR_CONNECTION_REFUSED|Unable to connect|hub never became ready|needs S_COLLAB_USER1_PASSWORD/u.test(String(error instanceof Error ? error.message : error)));
}
''')
sub('collabWaitForRow, runCollabE2eVerify };', 'collabWaitForRow, runCollabE2eCli, runCollabE2eVerify };')

old_v1 = 'import { runCollabE2eVerify } from "../🤝️collaboration/🟦️.ts";'
old_v2 = '''    if (segments[0] === "collab") {
      await runCollabE2eVerify();
      return;
    }'''
assert verify.count(old_v1) == 1 and verify.count(old_v2) == 1
verify = verify.replace(old_v1, 'import { runCollabE2eCli } from "../🤝️collaboration/🟦️.ts";').replace(old_v2, '''    if (segments[0] === "collab") {
      await runCollabE2eCli(segments.slice(1));
      return;
    }''')

open(COLLAB, "w", encoding="utf-8").write(text)
open(VERIFY, "w", encoding="utf-8").write(verify)
print("applied")
