#!/usr/bin/env python3
"""x-site apply: the course site build as graph tasks. Idempotent and anchored; writes nothing unless every hard edit resolves.

  apply.py [--check] [--emit-manifest OUT] [--force-files]

--check              report applied / pending / drift per edit, write nothing
--emit-manifest OUT  write the transformed project manifest to OUT (for the tests, COURSE_MANIFEST=OUT) and stop
--force-files        overwrite new files whose content differs
Exit codes: 0 done or already applied, 2 a hard anchor drifted (nothing written)."""
import json
import os
import re
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.abspath(os.environ.get("X_SITE_ROOT") or os.path.join(HERE, *[".."] * 9))
S = "🎓️teaching/🏛️architecture/📚️course"
PKG = f"{S}/📦️packages/🟦️typescript"
MANIFEST = f"{PKG}/📋️project.json"
SITE = f"{S}/🚀️deploy/🌐️site/🟦️.ts"
SCRIPT = f"{PKG}/📜️script.ts"
READINESS = f"{S}/🚀️deploy/⚖️readiness/🟦️.ts"
README = f"{S}/README.md"
STAGING_TEST = f"{S}/🧪️tests/🧪️deploy/🛑️cancellation-and-staging/🟦️.ts"
SEALED_TEST = f"{S}/🧪️tests/🧪️deploy/🌐️sealed-publication-and-bundle/🟦️.ts"
AGGREGATE = f"{S}/🧪️tests/🧪️deploy/🟦️.ts"
PUBLICATION_FIXTURE = f"{S}/🧫️fixtures/🌐️publication/🔣️.json"

RECEIPT = "{projectRoot}/.🧾️store/build.json"
BUNDLED = ["@dnd-kit/core", "@dnd-kit/sortable", "@dnd-kit/utilities", "@tailwindcss/vite", "@vitejs/plugin-react", "ajv", "i18next", "immer", "react", "react-dom", "react-i18next", "tailwindcss", "vite"]


class Edit:
    def __init__(self, path, label, hard=True):
        self.path, self.label, self.hard = path, label, hard

    def run(self, text):
        raise NotImplementedError


class Replace(Edit):
    def __init__(self, path, label, old, new, hard=True):
        super().__init__(path, label, hard)
        self.old, self.new = old, new

    def run(self, text):
        if (self.new and self.new in text) or (not self.new and self.old not in text):
            return text, "applied"
        if text.count(self.old) == 1:
            return text.replace(self.old, self.new, 1), "pending"
        return text, "drift"


class Region(Edit):
    def __init__(self, path, label, begin, end, new, marker, hard=True):
        super().__init__(path, label, hard)
        self.begin, self.end, self.new, self.marker = begin, end, new, marker

    def run(self, text):
        if (self.marker is None and self.begin not in text) or (self.marker is not None and self.marker in text):
            return text, "applied"
        start = text.find(self.begin)
        stop = text.find(self.end, start + 1) if start >= 0 else -1
        if start < 0 or stop < 0 or text.count(self.begin) != 1:
            return text, "drift"
        return text[:start] + self.new + text[stop:], "pending"


class Scoped(Edit):
    def __init__(self, path, label, begin, end, pairs, hard=True):
        super().__init__(path, label, hard)
        self.begin, self.end, self.pairs = begin, end, pairs

    def run(self, text):
        start = text.find(self.begin)
        stop = text.find(self.end, start + len(self.begin)) if start >= 0 else -1
        if start < 0 or stop < 0 or text.count(self.begin) != 1:
            return text, "drift"
        region, state = text[start:stop], "applied"
        for old, new in self.pairs:
            if new in region:
                continue
            if region.count(old) != 1:
                return text, "drift"
            region, state = region.replace(old, new, 1), "pending"
        return text[:start] + region + text[stop:], state


def read(path):
    with open(os.path.join(ROOT, path), encoding="utf-8") as handle:
        return handle.read()


def staged(name):
    with open(os.path.join(HERE, name), encoding="utf-8") as handle:
        return handle.read()


def build_tail():
    old = (
        '      "cache": false,\n      "inputs": [\n        "default",\n        {\n          "env": "PROCTOR_URL"\n        }\n      ],\n      "outputs": [],\n'
        '      "metadata": {\n        "semio": {\n          "cache": { "version": 1, "class": "managed-output", "reason": "outputs-in-managed-store" }\n        }\n      }\n    },\n    "publish": {'
    )
    external = ", ".join(json.dumps(name) for name in BUNDLED)
    new = (
        '      "cache": true,\n      "inputs": [\n        "default",\n        { "env": "PROCTOR_URL" },\n'
        f'        {{ "externalDependencies": [{external}] }},\n'
        '        "{workspaceRoot}/🎓️teaching/🏛️architecture/⚡️energy/**/❓️quiz/🔣️.json",\n        "{workspaceRoot}/🧰️framework/🔨️modules/🖼️assets/**/*"\n      ],\n'
        f'      "outputs": [\n        "{RECEIPT}"\n      ]\n    }},\n    "publish": {{'
    )
    return old, new


def manifest_edits():
    old, new = build_tail()
    return [
        Replace(MANIFEST, "build: receipt-cached with a complete key", old, new),
        Scoped(MANIFEST, "publish: depends on build, keyed by PROCTOR_URL", '    "publish": {\n', '    "test": {\n', [
            ('        "@semio-tech/ui-styling-tokens:generate"\n      ],', '        "@semio-tech/ui-styling-tokens:generate",\n        "@teaching/architecture-course:build"\n      ],'),
            ('      "cache": false,\n      "outputs": [', '      "cache": false,\n      "inputs": [\n        "default",\n        { "env": "PROCTOR_URL" }\n      ],\n      "outputs": ['),
        ]),
        Replace(MANIFEST, "deploy-check: compiler and site as graph tasks", '        "@teaching/architecture-course:typecheck"\n      ],', '        "@teaching/architecture-course:typecheck",\n        "@teaching/architecture-course:publish"\n      ],'),
    ]


SITE_REGION = '''/** 🏷️ The stable owner of the site product: `build` writes it and `publish` reads it, under the same mode. */
export const COURSE_SITE_OWNER = "architecture-course:site";

/** 🏗️ Plans the site product: its fixed owner, the original compiler mode and the only environment its bytes depend on, the mode and the proctor origin (never the source revision, a clock or a cache directory of the shell). */
export function courseSiteBuildPlan(repoRoot: string, bundleRoot: string, mode: BuildMode, environment: Readonly<Record<string, string | undefined>> = process.env): { workspace: string; root: string; config: string; mode: BuildMode; owner: string; environment: Record<string, string | undefined> } {
  return { workspace: repoRoot, root: bundleRoot, config: resolve(bundleRoot, "../../🏗️builder/🌐️vite/🟦️.ts"), mode, owner: COURSE_SITE_OWNER, environment: buildModeEnvironment(mode, { PROCTOR_URL: releaseProctorOrigin(environment) }) };
}

/** 📦️ Builds the site for the original caller's mode under the plan's owner. The product lives in the products store behind a store receipt, so a warm `build` is a cache hit and never reaches here. */
export async function buildCourseSite(repoRoot: string, bundleRoot: string, invocation: ScriptInvocation): Promise<void> {
  const mode = courseOriginalBuildModeV1(invocation, "dev");
  await buildViteArtifact({ ...courseSiteBuildPlan(repoRoot, bundleRoot, mode), invocation, control: scriptCompilerAdmissionControlV1(invocation, "vite:course-site", 0), signal: invocation.control.signal, timeoutMs: courseBudgetMs(invocation) });
}

/** 🚚️ Reads the release site `build` published, under immutable custody, and exports a separately validated CDN artifact. It builds nothing: the cached `build` task is its prerequisite. */
export async function publishCourseSite(repoRoot: string, bundleRoot: string, invocation: ScriptInvocation): Promise<void> {
  courseOriginalBuildModeV1(invocation, "release");
  const proctorOrigin = releaseProctorOrigin();
  say(`staging the site built for ${COURSE_SITE_ORIGIN} against ${proctorOrigin}`);
  await withViteArtifactV1(repoRoot, "release", COURSE_SITE_OWNER, invocation, scriptCompilerAdmissionControlV1(invocation, "architecture-course:site-reader", 0), async artifact => {
    await artifact.recheck();
    stageCourseSiteArtifact(artifact.path, join(bundleRoot, "dist", "release", COURSE_PAGES_DIRECTORY), proctorOrigin, invocation.control.signal);
  });
  if (proctorOrigin !== COURSE_PROCTOR_ORIGIN) say(`REHEARSAL artifact for ${proctorOrigin}; do not upload it to the production host`);
}

'''

READINESS_REGION = '''/** 🪜️ The steps of `deploy-check` in order for a package with the nx `targets`: the cheap proof first, no drift, then the course in the Rust core, the image, its check, the stack and, last, the end-to-end gate when the package has one. The compiler and the site are graph prerequisites ({@link DEPLOY_CHECK_GRAPH_TASKS}), not steps. */
export function deployCheckSteps(targets: Readonly<Record<string, unknown>>): readonly DeployCheckStep[] {
  return DEPLOY_CHECK_STEPS.filter((step) => step !== "end-to-end gate" || "test-e2e" in targets);
}

/** ⚖️ `deploy-check [--tag <tag>]` — everything that must hold before the owner deploys. The tool first runs its two cached prerequisites ({@link DEPLOY_CHECK_GRAPH_TASKS}): the package's sources against the compiler, and the site built and verified as the CDN artifact. The gate then runs, in the order of {@link deployCheckSteps}, each step announced with its number and time and stopping at the first failure or at Ctrl-C: no drift from `🔣️.json`, the course valid in the Rust core, the image built, the image check, the stack check, and the package's end-to-end gate. Nothing is pushed or uploaded. */
export async function checkCourseDeployment(repoRoot: string, bundleRoot: string, checks: Readonly<{ endToEnd: () => void | Promise<void> }>, segments: readonly string[], invocation: ScriptInvocation): Promise<void> {
  courseOriginalBuildModeV1(invocation, "release");
  if (segments.includes("--jobs")) throw Error("The native compiler owner controls concurrency; --jobs is not a deployment option");
  const tagged = flag(segments, "--tag") ? ["--tag", flag(segments, "--tag")!] : [];
  const targets = (JSON.parse(readFileSync(join(bundleRoot, "📋️project.json"), "utf8")) as { targets: Record<string, { dependsOn?: string[] }> }).targets;
  const missing = DEPLOY_CHECK_GRAPH_TASKS.filter((task) => !(targets["deploy-check"]?.dependsOn ?? []).includes(task));
  if (missing.length > 0) throw new Error(`deploy-check must depend on ${missing.join(" and ")}: the compiler and the site are graph tasks, never repeated here`);
  const run: { readonly [S in DeployCheckStep]: () => Promise<void> | void } = {
    "no drift from 🚀️deploy/🔣️.json": () => {
      const drift = deploymentDrift(repoRoot);
      if (drift.length > 0) throw new Error(`the deployment drifted:\\n- ${drift.join("\\n- ")}`);
    },
    "course valid in the Rust core": () => checkCourse(repoRoot, invocation),
    "proctor image built": () => buildCourseImage(repoRoot, tagged, invocation),
    "proctor image checked": () => checkCourseImage(repoRoot, tagged, invocation),
    "stack checked": () => checkCourseStack(tagged, invocation),
    "end-to-end gate": checks.endToEnd,
  };
  const steps = deployCheckSteps(targets);
  const { signal, release } = interruption(invocation);
  const took: string[] = [];
  try {
    for (const [index, label] of steps.entries()) {
      if (signal.aborted) throw new Error("cancelled");
      say(`step ${index + 1}/${steps.length}: ${label}`);
      const started = Date.now();
      await run[label]();
      took.push(`${label} (${Math.round((Date.now() - started) / 1000)} s)`);
    }
  } finally {
    release();
  }
  for (const warning of deploymentWarnings()) say(`WARNING: ${warning}`);
  say(`ready to deploy: ${took.join("; ")}`);
}
'''

README_GATE = '''The compiler and the site are not repeated by the gate. They are graph prerequisites of the target, so the tool runs them
first, caches them and reports each as a task of its own:

- `typecheck`: the sources pass the compiler: the entry, the Vite configuration, the stack, the deploy verbs, the
  end-to-end gate and every test and spec;
- `publish`, on top of the cached `build`: the site builds for the production proctor and is the CDN artifact: entry
  points, a document sealed by its Content-Security-Policy, hashed assets, no source map or development leftover, within
  the size budget (`COURSE_SITE_BUDGET`: what the document links — the entry script 300 000 B and the stylesheet
  60 000 B gzip — and 5 MB on disk; a run with its results and the pets are lazy chunks the first screen does not wait
  for, so they are not charged: on 2026-10-03 the entry weighed 253 374 B, the run chunk 13 867 B, the results chunk
  3 217 B and the quiz's half of the pets' glue 1 499 B).

The gate itself then runs six steps:

1. nothing drifted from `🚀️deploy/🔣️.json` (hosts, image, port, pinned base images and actions, targets named in docs);
2. the course and every quiz are valid in the Rust core (`proctor check`);
3. the proctor image builds;
4. the image is what it claims (labels, unprivileged account, no shell, the stack files inside) and serves the API as the
   site meets it, read-only and without capabilities;
5. the stack works exactly as a host gets it: the two stack files copied out of the image, TLS through Caddy, the
   hardening Docker applied, a backup while serving, an update without a failed request, a restore, a supplied
   certificate served instead of Caddy's own;
6. the end-to-end gate (`test-e2e`): the release build in a real browser against a production-mode proctor.

'''


def code_edits():
    return [
        Replace(SITE, "site: mode type import", 'import { buildModeEnvironment } from "../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎚️mode/🟦️.ts";', 'import { buildModeEnvironment, type BuildMode } from "../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎚️mode/🟦️.ts";'),
        Replace(SITE, "site: budget import", 'import { courseOriginalBuildModeV1 } from "../../🏗️builder/🎚️mode/🟦️.ts";', 'import { courseOriginalBuildModeV1 } from "../../🏗️builder/🎚️mode/🟦️.ts";\nimport { courseBudgetMs } from "../../🏗️builder/⏱️budget/🟦️.ts";'),
        Region(SITE, "site: one builder, a publish that only stages", "/** 🏗️ Selects the release product's stable owner", "/** 📤️ Stages only validated public files", SITE_REGION, "export function courseSiteBuildPlan"),
        Replace(SCRIPT, "router: no vite build wrapper", "runViteBunxDev, runViteBuild }", "runViteBunxDev }"),
        Replace(SCRIPT, "router: site builder import", "import { buildCourseReleaseSite, publishCourseSite }", "import { buildCourseSite, publishCourseSite }"),
        Region(SCRIPT, "router: build", "class BuildScript extends BundleScript {", "class TestScript extends BundleScript {", '''class BuildScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length) throw new Error("Course build accepts no arguments");
    await buildCourseSite(this.repoRoot, this.root, this.invocation);
  }
}
''', "await buildCourseSite(this.repoRoot, this.root, this.invocation);"),
        Region(SCRIPT, "router: publish", "class PublishScript extends BundleScript {", "class DockerImageBuildScript extends BundleScript {", '''class PublishScript extends BundleScript {
  async run(): Promise<void> {
    await publishCourseSite(this.repoRoot, this.root, this.invocation);
  }
}
''', "await publishCourseSite(this.repoRoot, this.root, this.invocation);"),
        Region(SCRIPT, "router: deploy-check", "class DeployCheckScript extends BundleScript {", "const router = new ScriptRouter", '''class DeployCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await checkCourseDeployment(this.repoRoot, this.root, {
      endToEnd: () => runCourseEndToEnd(this.invocation, this.repoRoot, this.root, fileURLToPath(import.meta.url), playgroundPortEnv("architecture-course"), ["--serial", "--"]),
    }, segments, this.invocation);
  }
}

''', "checkCourseDeployment(this.repoRoot, this.root, {\n      endToEnd:"),
        Replace(READINESS, "readiness: no site import", 'import { publishCourseSite } from "../🌐️site/🟦️.ts";\n', ""),
        Replace(READINESS, "readiness: steps", 'const DEPLOY_CHECK_STEPS = ["no drift from 🚀️deploy/🔣️.json", "sources type-checked", "course valid in the Rust core", "site built and verified as the CDN artifact", "proctor image built", "proctor image checked", "stack checked", "end-to-end gate"] as const;',
                'const DEPLOY_CHECK_STEPS = ["no drift from 🚀️deploy/🔣️.json", "course valid in the Rust core", "proctor image built", "proctor image checked", "stack checked", "end-to-end gate"] as const;\n\n/** 🕸️ The graph tasks `deploy-check` depends on instead of repeating them: the compiler over the sources, and the site built and verified as the CDN artifact. */\nexport const DEPLOY_CHECK_GRAPH_TASKS = ["@teaching/architecture-course:typecheck", "@teaching/architecture-course:publish"] as const;'),
        Region(READINESS, "readiness: graph-aware gate", "/** 🪜️ The steps of", "//#endregion ⚖️Readiness", READINESS_REGION, "deploy-check must depend on"),
        Region(STAGING_TEST, "test: plan moved to the graph test", '  it("builds publication as a release even when the dashboard selected prototype", () => {', '  for (const row of sitePublication.cases) it(`stages ${row.id}', "", None),
        Replace(STAGING_TEST, "test: no removed plan import", "import { courseReleaseBuildPlan, siteArtifactProblems, stageCourseSiteArtifact }", "import { siteArtifactProblems, stageCourseSiteArtifact }"),
        Replace(SEALED_TEST, "test: steps", '''expect(steps).toEqual(["no drift from 🚀️deploy/🔣️.json", "sources type-checked", "course valid in the Rust core", "site built and verified as the CDN artifact", "proctor image built", "proctor image checked", "stack checked", "end-to-end gate"]);''', '''expect(steps).toEqual(["no drift from 🚀️deploy/🔣️.json", "course valid in the Rust core", "proctor image built", "proctor image checked", "stack checked", "end-to-end gate"]);'''),
        Replace(SEALED_TEST, "test: steps without e2e", '''expect(deployCheckSteps({})).toEqual(steps.filter((step) => step !== "sources type-checked" && step !== "end-to-end gate"));''', '''expect(deployCheckSteps({})).toEqual(steps.filter((step) => step !== "end-to-end gate"));'''),
        Replace(SEALED_TEST, "test: title", "checks the sources against the compiler early in the readiness gate, before anything is built, and ends with the end-to-end gate", "reaches the compiler and the site through the graph before the readiness gate starts, and ends with the end-to-end gate"),
        Replace(AGGREGATE, "tests: register the graph tests", 'import { registerSiteBuildTests } from "./🏗️site-build/🟦️.ts";', 'import { registerSiteBuildTests } from "./🏗️site-build/🟦️.ts";\nimport { registerSiteGraphTests } from "./🕸️site-graph/🟦️.ts";'),
        Replace(AGGREGATE, "tests: run the graph tests", "  registerSiteBuildTests();\n", "  registerSiteBuildTests();\n  registerSiteGraphTests();\n"),
        Replace(PUBLICATION_FIXTURE, "fixture: the plan moved to the graph fixture", '  "build": { "mode": "release", "owner": "architecture-course:site" },\n', ""),
        Region(README, "README: readiness gate", "1. nothing drifted from `🚀️deploy/🔣️.json`", "It ends with `ready to deploy`", README_GATE, "The compiler and the site are not repeated by the gate"),
        Replace(README, "README: build row (en)", "| `@teaching/architecture-course:build` | the production site for the proctor named by `PROCTOR_URL` (else the production proctor) | assets, tokens | never | |", "| `@teaching/architecture-course:build` | the site for the proctor named by `PROCTOR_URL` (else the production proctor), in the caller's build mode, published into the products store | assets, tokens | yes: its output is the store receipt `.🧾️store/build.json`; a hit restores the receipt and the backend verifies the store content, anything else rebuilds | |", hard=False),
        Replace(README, "README: publish row (en)", "| `@teaching/architecture-course:publish` | stages the CDN artifact at `dist/release/pages/course` (release mode) | assets, tokens | never | |", "| `@teaching/architecture-course:publish` | verifies the release site `build` published and stages the CDN artifact at `dist/release/pages/course` (release mode); builds nothing itself | assets, tokens, `build` | never (the workspace name policy forbids replaying `publish`; `build` is the cached part) | |", hard=False),
        Replace(README, "README: deploy-check row (en)", "| `@teaching/architecture-course:deploy-check` | the [readiness gate](#readiness-gate) | assets, tokens, `typecheck` | never | |", "| `@teaching/architecture-course:deploy-check` | the [readiness gate](#readiness-gate) | assets, tokens, `typecheck`, `publish` | never | |", hard=False),
        Replace(README, "README: cached targets (en)", "declares explicit inputs: `test` and `typecheck`.", "declares explicit inputs: `test`, `typecheck` and `build` (whose output is a store receipt).", hard=False),
        Replace(README, "README: build row (de)", "| `@teaching/architecture-course:build` | die Produktionsseite für den Proctor aus `PROCTOR_URL` (sonst der Produktions-Proctor) | assets, tokens | nie | |", "| `@teaching/architecture-course:build` | die Seite für den Proctor aus `PROCTOR_URL` (sonst der Produktions-Proctor) im Build-Modus des Aufrufers, im Produkte-Store veröffentlicht | assets, tokens | ja: Ausgabe ist der Store-Beleg `.🧾️store/build.json`; ein Treffer stellt den Beleg wieder her, das Werkzeug prüft den Store-Inhalt, sonst wird neu gebaut | |", hard=False),
        Replace(README, "README: publish row (de)", "| `@teaching/architecture-course:publish` | legt das CDN-Artefakt unter `dist/release/pages/course` bereit (Release-Modus) | assets, tokens | nie | |", "| `@teaching/architecture-course:publish` | prüft die Release-Seite, die `build` veröffentlicht hat, und legt das CDN-Artefakt unter `dist/release/pages/course` bereit (Release-Modus); baut selbst nichts | assets, tokens, `build` | nie (die Namensregel des Workspace verbietet das Wiedergeben von `publish`; `build` ist der gecachte Teil) | |", hard=False),
        Replace(README, "README: deploy-check row (de)", "| `@teaching/architecture-course:deploy-check` | das Bereitschaftstor (*Readiness gate*, englisch oben) | assets, tokens, `typecheck` | nie | |", "| `@teaching/architecture-course:deploy-check` | das Bereitschaftstor (*Readiness gate*, englisch oben) | assets, tokens, `typecheck`, `publish` | nie | |", hard=False),
        Replace(README, "README: cached targets (de)", "ausdrückliche Eingaben deklariert: `test` und `typecheck`.", "ausdrückliche Eingaben deklariert: `test`, `typecheck` und `build` (dessen Ausgabe ein Store-Beleg ist).", hard=False),
    ]


NEW_FILES = [
    ("site-graph.test.ts", f"{S}/🧪️tests/🧪️deploy/🕸️site-graph/🟦️.ts"),
    ("site-graph.schema.json", f"{S}/🧪️tests/🧪️deploy/🕸️site-graph/🧬️schema/🔣️.json"),
    ("site-graph.fixture.json", f"{S}/🧫️fixtures/🕸️site-graph/🔣️.json"),
]


def main(argv):
    check = "--check" in argv
    force = "--force-files" in argv
    edits = [edit for edit in manifest_edits() + code_edits() if edit is not None]
    if "--emit-manifest" in argv:
        target = argv[argv.index("--emit-manifest") + 1]
        text = read(MANIFEST)
        for edit in manifest_edits():
            text, state = edit.run(text)
            if state == "drift":
                print(f"drift: {edit.label}", file=sys.stderr)
                return 2
        json.loads(text)
        with open(target, "w", encoding="utf-8") as handle:
            handle.write(text)
        return 0
    texts, states, drifted = {}, [], False
    for edit in edits:
        texts.setdefault(edit.path, read(edit.path))
        texts[edit.path], state = edit.run(texts[edit.path])
        states.append((edit, state))
        print(f"{state:8} {'hard' if edit.hard else 'soft'} {edit.label}")
        drifted = drifted or (state == "drift" and edit.hard)
    for source, target in NEW_FILES:
        current = read(target) if os.path.exists(os.path.join(ROOT, target)) else None
        state = "applied" if current == staged(source) else "pending" if current is None else "drift"
        print(f"{state:8} hard    new file {target.split('/')[-2]}")
        drifted = drifted or (state == "drift" and not force)
    if drifted:
        print("a hard anchor drifted: nothing written", file=sys.stderr)
        return 2
    if check:
        return 0
    json.loads(texts[MANIFEST])
    for path, text in texts.items():
        if text != read(path):
            with open(os.path.join(ROOT, path), "w", encoding="utf-8") as handle:
                handle.write(text)
    for source, target in NEW_FILES:
        full = os.path.join(ROOT, target)
        os.makedirs(os.path.dirname(full), exist_ok=True)
        with open(full, "w", encoding="utf-8") as handle:
            handle.write(staged(source))
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
