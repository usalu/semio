"""Increment 3 (c4): the build catalog; the German texts are in specs/de-build.txt, the skeleton in specs/build.spec."""
from edits_catalogs import migrate_catalog
from lib import *
from scopes3 import catalog_config

B = L + "/🏗️build"


def prepare(tree):
    tree.edit(B + "/🛠️cmake/🟦️.ts", 'console.log("[cpp] Completed the "+mode+" native graph")', "console.log(`[cpp] Completed the ${mode} native graph`)")
    tree.edit(B + "/🔧️tools/🏗️bazel/📜️script.ts", 'process.stderr.write("[bazel] Cleanup refusal retained after primary: "+message+"\\n")', "process.stderr.write(`[bazel] Cleanup refusal retained after primary: ${message}\\n`)")
    schema = B + "/🏡️resident/🧬️schema/🔣️.json"
    tree.edit(schema, '            "service-down": "No resident service is running for this worktree.",\n', '            "service-down": "No resident service is running for this worktree.",\n            "service-required": "--service=require: no resident service answered ({reason}).",\n')
    tree.edit(schema, '            "service-down": "Für dieses Worktree läuft kein residenter Dienst.",\n', '            "service-down": "Für dieses Worktree läuft kein residenter Dienst.",\n            "service-required": "--service=require: Kein residenter Dienst hat geantwortet ({reason}).",\n')
    client = B + "/🏡️resident/🚪️client/🔁️repo/🟨️.mjs"
    tree.edit(client, "import {POLICY_V1,SUPPORTED_LANGUAGES_V1} from", "import {POLICY_V1,SUPPORTED_LANGUAGES_V1,preferredLanguageV1,renderTextV1} from")
    tree.edit(client, 'stderr.write("--service=require: no resident service answered ("+result.reason+")\\n");', 'stderr.write(renderTextV1("service-required",{reason:result.reason},languages)[preferredLanguageV1(languages)]+"\\n");')


def run(tree):
    prepare(tree)
    migrate_catalog(tree, "build", catalog_config("build", tree))
