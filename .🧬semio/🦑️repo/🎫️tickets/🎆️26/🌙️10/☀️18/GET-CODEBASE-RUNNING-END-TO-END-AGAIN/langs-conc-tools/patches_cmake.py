# 🛠 CMake: the driver selects the checksum-pinned CMake and Ninja of the tools partition (provisioned once by the setup operation) before any program lookup, so no agent builds with whatever the host package manager installed.
from layout import *
from patchlib import chain

CMAKE = B + "/🛠️cmake/🏃️execution/🟦️.ts"

def driver(text):
    return chain(text, [
        ("cmake-import", 'import{withRepositoryToolingStoreV1}from"../../../⚡️caching/💾️storage/🔒️enforcement/🛠️tools/🟦️.ts";\n',
         'import{withRepositoryToolingStoreV1}from"../../../⚡️caching/💾️storage/🔒️enforcement/🛠️tools/🟦️.ts";\nimport{pinnedNativeEnvironmentV1}from"../../../⚡️caching/🚀️bootstrap/🛠️tools/🧰️native/🟦️.ts";\nimport{systemNativeToolPortV1}from"../../../⚡️caching/🚀️bootstrap/🛠️tools/🧰️native/🔌️port/🟦️.ts";\n'),
        ("cmake-pinned", 'environment=Object.freeze(activated.environment),cmake=program("cmake",environment)',
         'environment=Object.freeze(await pinnedNativeEnvironmentV1(activated.environment,tools.root,systemNativeToolPortV1(root,authority.network.offline),toolControl.signal,operation==="setup")),cmake=program("cmake",environment)'),
    ])

def edits():
    return [(CMAKE, driver)]
