"""🧪️ Round 2, concern 4: tests follow the renamed runtime field and the materialized schema; none of them pins a manager literal any more."""
def declare(edit, paths):
    D = paths.DEPENDENCIES
    G = D + "/🧾️generation/🧪️tests/🟦️.ts"
    edit(G, "tests-runtime-fixture", "generation2", 'const runtime:GenerationRuntimeV1={bun:"0.0.0"', 'const runtime:GenerationRuntimeV1={manager:"0.0.0"')
    edit(G, "tests-runtime-moved", "generation2", '{...runtime,bun:"9.9.9"}', '{...runtime,manager:"9.9.9"}')
    edit(D + "/💾️storage/🧪️tests/👓️reading/🟦️.ts", "tests-clone-schema", "storage2", "contractData=structuredClone(schema)", 'contractData=(await import("../../🧱️schemas/🟦️.ts")).cloneJavascriptInstallSchemaV1(schema)')
    RAW = 'JSON.parse(readFileSync(new URL("../../💾️storage/🧬️schema/📥️install.json",import.meta.url),"utf8"))'
    edit(G, "tests-import-materialized", "generation2", 'import {test,expect} from "bun:test";', 'import {test,expect} from "bun:test";\nimport {javascriptInstallSchemaV1 as materializedInstallSchema} from "../../💾️storage/🧱️schemas/🟦️.ts";')
    edit(G, "tests-materialized-schema", "generation2", RAW, "materializedInstallSchema", 2)
    I = D + "/🧾️incremental/🧪️tests/🟦️.ts"
    edit(I, "tests-import-materialized", "storage2", 'import {test,expect} from "bun:test";', 'import {test,expect} from "bun:test";\nimport {javascriptInstallSchemaV1 as materializedInstallSchema} from "../../💾️storage/🧱️schemas/🟦️.ts";')
    edit(I, "tests-materialized-schema", "storage2", RAW, "materializedInstallSchema")
    edit(G, "tests-pin-manager-path", "generation2", "packageManagerV1.configurationNames(", "installSchema.$defs.ManagerProtocol.manager.configurationNames(", 4)
    edit(G, "tests-pin-home-annotation", "generation2", ".map(name=>path.join(home,name))", ".map((name:string)=>path.join(home,name))")
    edit(G, "tests-pin-config-annotation", "generation2", ".map(name=>path.join(config,name))", ".map((name:string)=>path.join(config,name))")
