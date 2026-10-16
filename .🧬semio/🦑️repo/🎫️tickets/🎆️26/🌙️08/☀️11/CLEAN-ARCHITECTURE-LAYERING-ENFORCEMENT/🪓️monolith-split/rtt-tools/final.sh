#!/bin/zsh
D=$(cd "$(dirname "$0")" && pwd)
V="$D/vitest.sh"
args=$(node --input-type=module -e '
const cfg=(await import("file://'"$D"'/cfg-ec.mjs")).default;
console.log(cfg.topics.map(t=>"tests/"+t.folder+"/").join(" "));
')
$V long final-ec ${=args}
$V long final-pr 'PluginRuntime/'
$V exhaustive final-tw 'UiDocumentStore/'
touch "$D/../🗑️generated/rtt-final.done"
