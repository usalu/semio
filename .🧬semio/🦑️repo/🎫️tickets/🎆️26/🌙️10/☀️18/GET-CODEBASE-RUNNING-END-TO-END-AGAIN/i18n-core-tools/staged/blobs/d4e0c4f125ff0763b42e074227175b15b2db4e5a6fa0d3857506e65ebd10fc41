import { BUNDLE_DATE_SECTION_RE, BUNDLE_WIP_SUBJECT_RE, type CommitBundleDateSection, type CommitBundleSection } from "../🧾️bundle-model/🟦️.ts";
import { bulletEmojiValidationError, MICRO_COMMIT_BULLET_RE } from "../🔸️bullets/🟦️.ts";
import { normalizeBundleDateLine } from "../📅️dates/🟦️.ts";
import { bundleScopeLabelError, isBundleScopeLine, normalizeBundleScopeLabel } from "../🏷️labels/🟦️.ts";
import {refuseCommitV1} from "../📖️texts/🟦️.ts";

/** 🚫️Validates stdin is bundle body only, not a full commit message. */
export function commitBundleBodyError(text: string): string | null {
  const first =
    text
      .trim()
      .split("\n")
      .find((l) => l.trim().length > 0)
      ?.trim() ?? "";
  if (BUNDLE_WIP_SUBJECT_RE.test(first)) {
    return "commit: stdin must not include the bundle subject (…🔀️) — script adds it";
  }
  if (/^🐙️|^🧑️/.test(first) && first.includes("🚩️")) {
    return "commit: stdin must not include micro-commit subject lines";
  }
  if (/^📊️metric/m.test(text)) {
    return "commit: stdin must not include the 📊️metric footer — script adds it";
  }
  if (/^🎆️\d{2}🌙️\d{2}☀️\d{2}📊️metric/m.test(text)) {
    return "commit: stdin must not include per-day 📊️metric — script adds it to each 🎆️ line";
  }
  return null;
}

function validateBulletLine(b: string): void {
  if (!MICRO_COMMIT_BULLET_RE.test(b)) {
    throw refuseCommitV1("invalid","body.bullet-must-start-with-then",{bullet:String(b)});
  }
  const err = bulletEmojiValidationError([b]);
  if (err) throw new Error(err.replace(/^micro-commit:/, "commit:"));
  if (BUNDLE_DATE_SECTION_RE.test(b.trim())) {
    throw refuseCommitV1("failed","body.use-a-date-section-line");
  }
}

/** 📦️Parses LLM bundle body (emoji-only scope lines, dates, bullets). */
export function parseCommitBundleBody(text: string): CommitBundleSection[] {
  const bundles: CommitBundleSection[] = [];
  let current: CommitBundleSection | null = null;
  let dateSection: CommitBundleDateSection | null = null;

  for (const raw of text.split("\n")) {
    const line = raw.trim();
    if (!line || line.startsWith("#")) continue;
    if (line.startsWith("📊️metric") || line.startsWith("Signed-off-by:")) continue;

    const dateCandidate = normalizeBundleDateLine(line);
    if (BUNDLE_DATE_SECTION_RE.test(dateCandidate)) {
      if (!current) throw refuseCommitV1("failed","body.date-section-before-bundle-scope",{line:String(line)});
      if (dateSection) current.dates.push(dateSection);
      dateSection = { dateLine: dateCandidate, bullets: [] };
      continue;
    }

    if (dateSection) {
      if (isBundleScopeLine(line)) {
        const scopeErr = bundleScopeLabelError(line);
        if (scopeErr) throw new Error(scopeErr);
        if (current) {
          current.dates.push(dateSection);
          bundles.push(current);
        }
        current = { label: normalizeBundleScopeLabel(line), dates: [] };
        dateSection = null;
        continue;
      }
      validateBulletLine(line);
      dateSection.bullets.push(line);
      continue;
    }

    if (isBundleScopeLine(line)) {
      const scopeErr = bundleScopeLabelError(line);
      if (scopeErr) throw new Error(scopeErr);
      if (current) bundles.push(current);
      current = { label: normalizeBundleScopeLabel(line), dates: [] };
      continue;
    }

    if (!current) throw refuseCommitV1("invalid","body.expected-emoji-bundle-scope-got",{line:String(line)});
    throw refuseCommitV1("invalid","body.expected-yy-mm-dd-date",{line:String(line)});
  }

  if (current) {
    if (dateSection) current.dates.push(dateSection);
    bundles.push(current);
  }
  if (bundles.length === 0) throw refuseCommitV1("missing","body.at-least-one-emoji-bundle");
  for (const b of bundles) {
    if (b.dates.length === 0) throw refuseCommitV1("invalid","body.bundle-needs-at-least-one",{label:String(b.label)});
    for (const d of b.dates) {
      if (d.bullets.length === 0) throw refuseCommitV1("invalid","body.in-needs-at-least-one",{dateLine:String(d.dateLine),label:String(b.label)});
    }
  }
  return bundles;
}
