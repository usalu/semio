# 🐍 Python: one isolated environment per project from the toolchain plan, interpreters only from the managed pinned install directory, one interpreter series per project (3.14, the energy oracle 3.12) written identically in every manifest, lock and schema.
from layout import *
from patchlib import chain

ORACLE = ROOT + "/✏️s/🔌️plugins/🔋️energy/🔮️oracles"
STYLING_PY = ROOT + "/🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🐍️python"
BUILDER = L + "/🎨️styling/🏗️builder/🐍️python"
BUILDER_SCHEMA = ROOT + "/🧰️framework/🔨️modules/🖱️ui/🎨️styling/🏗️builder/🐍️python/🧬️schema/🔣️.json"

def oracle(text):
    return chain(text, [
        ("oracle-import", 'import { BundleScript } from "@semio-tech/framework/process/routing";\n', 'import { BundleScript } from "@semio-tech/framework/process/routing";\nimport { pythonEnvironmentOfProjectV1 } from "@semio-tech/framework/process/toolchains";\n'),
        ("oracle-run", '  env.PYTHONDONTWRITEBYTECODE = "1";\n', '  env.PYTHONDONTWRITEBYTECODE = "1";\n  Object.assign(env, pythonEnvironmentOfProjectV1(repoRoot, "energy-oracle").variables);\n'),
        ("oracle-deps", '    runCmd("uv", ["sync", "--locked", "--project", this.root], { cwd: this.root });\n',
         '    const plan = pythonEnvironmentOfProjectV1(this.repoRoot, "energy-oracle"), env = { ...process.env, ...plan.variables };\n    runCmd("uv", [...plan.installArguments], { cwd: this.repoRoot, env });\n    runCmd("uv", [...plan.syncArguments], { cwd: this.repoRoot, env });\n'),
    ])

def builder(text):
    return chain(text, [
        ("builder-imports", 'import{observeStylingPythonEnvironmentV1}from"./📋️inventory/🟦️.ts";\n',
         'import{observeStylingPythonEnvironmentV1}from"./📋️inventory/🟦️.ts";\nimport{repositoryCachePartitionRootV1}from"../../../📦️packages/🟦️typescript/💾️storage/🔒️enforcement/📦️budget/📥️opening/🟦️.ts";\nimport{pythonInterpreterDirectoryV1}from"../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🧰️toolchains/🟦️.ts";\n'),
        ("builder-variables", 'UV_NO_CONFIG:"1",UV_NO_MANAGED_PYTHON:"1",UV_PYTHON_DOWNLOADS:"never",UV_CACHE_DIR:join(scratch.directory,"uv-cache"),UV_PYTHON_INSTALL_DIR:join(scratch.directory,"python-interpreters")})',
         'UV_NO_CONFIG:"1",UV_PYTHON_DOWNLOADS:"manual",UV_CACHE_DIR:join(scratch.directory,"uv-cache"),UV_PYTHON_INSTALL_DIR:pythonInterpreterDirectoryV1(repositoryCachePartitionRootV1(workspace,"tools"))})'),
        ("builder-sync", '"sync","--no-config","--no-python-downloads","--no-managed-python","--locked","--group","build","--no-install-project"', '"sync","--no-config","--no-python-downloads","--managed-python","--locked","--group","build","--no-install-project"'),
        ("builder-run", '"run","--no-config","--no-python-downloads","--no-managed-python","--locked","--no-sync","--offline"', '"run","--no-config","--no-python-downloads","--managed-python","--locked","--no-sync","--offline"'),
        ("builder-find", 'const found=(await command(uv,["python","find","--no-config","--no-python-downloads","--no-managed-python",policy.pythonVersion],scratch.directory,bounded,scratch.control)).trim();if(!isAbsolute(found)||/[\\r\\n\\0]/u.test(found))throw Error("One actual system interpreter required");',
         'if(request.operation==="deps"&&!authority.network.offline)await command(uv,["python","install","--no-config",policy.pythonVersion],scratch.directory,bounded,scratch.control);const found=(await command(uv,["python","find","--no-config","--no-python-downloads","--managed-python",policy.pythonVersion],scratch.directory,bounded,scratch.control)).trim();if(!isAbsolute(found)||/[\\r\\n\\0]/u.test(found))throw Error("One actual managed interpreter required");'),
    ])

def styling_pyproject(text):
    return chain(text, [("styling-requires", 'requires-python = ">=3.12"\n', 'requires-python = ">=3.14,<3.15"\n')])

def styling_lock(text):
    return chain(text, [("styling-lock", 'requires-python = ">=3.12"\n', 'requires-python = "==3.14.*"\n')])

def builder_schema(text):
    return chain(text, [("styling-schema", '"requires-python": ">=3.12"', '"requires-python": ">=3.14,<3.15"')])

def edits():
    return [
        (ORACLE + "/🏃️execution/🟦️.ts", oracle),
        (BUILDER + "/🟦️.ts", builder),
        (STYLING_PY + "/pyproject.toml", styling_pyproject),
        (STYLING_PY + "/uv.lock", styling_lock),
        (BUILDER_SCHEMA, builder_schema),
    ]
