import sys
sys.path.insert(0,".")
from anchored import patch
L="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/"
patch(L+"🏗️build/🕸️graph/🟦️.ts",[
('''const requireCurrentGraphModuleV1=():void=>{requireCurrentFormatStampsV1(graphFormatStampsV1,"graph generation");if(implementationPinnedV1())return;''','''const requireCurrentGraphModuleV1=():void=>{if(implementationPinnedV1())return;'''),
('''signal?.throwIfAborted();return discoverBuildGraphPartV1(root,registry,invocation,control,state.files,''','''signal?.throwIfAborted();requireCurrentFormatStampsV1(graphFormatStampsV1,"graph generation");return discoverBuildGraphPartV1(root,registry,invocation,control,state.files,'''),
])
