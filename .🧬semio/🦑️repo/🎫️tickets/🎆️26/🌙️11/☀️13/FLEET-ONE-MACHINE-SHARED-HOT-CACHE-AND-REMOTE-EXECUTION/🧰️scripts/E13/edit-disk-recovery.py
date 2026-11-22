B="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌡️host-capacity/🧹️disk-recovery/"
def sub(path,old,new,count=1):
    s=open(B+path,encoding="utf8").read()
    assert s.count(old)==count,(path,old[:70],s.count(old))
    open(B+path,"w",encoding="utf8").write(s.replace(old,new))
sub("🧬️schema/🔣️.json",'"collector": {"enum": ["ledger", "cas-lru", "staging-sweep", "image-sweep", "age-prune", "log-rotate", "image-reset", "store-compaction", "manual"]}','"collector": {"enum": ["ledger", "cas-lru", "staging-sweep", "image-sweep", "age-prune", "log-rotate", "native-shards", "image-reset", "store-compaction", "manual"]}')
sub("🧬️schema/🔣️.json",'''        {"id": "closure-layers", "priority": 30,''','''        {"id": "nx-native-data", "priority": 25, "class": "derivable", "relief": "image", "root": "{workspace}/.cache/b/n/m", "collector": "native-shards", "command": "repo cache gc", "bound": {"en": "The Nx file map (nx_files.nxt) inside each native Nx shard, removed when the shard cannot rewrite it next to the old copy and no run holds the shard; Nx rebuilds it.", "de": "Die Nx-Dateikarte (nx_files.nxt) in jedem nativen Nx-Shard, entfernt, wenn der Shard sie nicht neben die alte Kopie schreiben kann und kein Lauf ihn hält; Nx baut sie neu auf."}},
        {"id": "closure-layers", "priority": 30,''')
sub("🟦️.ts",'"age-prune"|"log-rotate"|"image-reset"','"age-prune"|"log-rotate"|"native-shards"|"image-reset"')
sub("🔌️real/🟦️.ts",'''   case "leaked-images":return''','''   case "nx-native-data":return{...base,measure:async()=>readNativeShardsV1(context.workspace).reduce((sum,fact)=>sum+planNativeShardReliefV1([fact]).reduce((bytes,item)=>bytes+item.bytes,0),0),collect:async({signal})=>(await relieveNativeShardsV1({facts:readNativeShardsV1(context.workspace),hold:nativeShardHoldV1(context.workspace),language:context.language??"en",dryRun,signal,...context.report===undefined?{}:{}})).freedBytes};
   case "leaked-images":return''')
sub("🔌️real/🟦️.ts",'''import {fileURLToPath} from "node:url";''','''import {fileURLToPath} from "node:url";
import {planNativeShardReliefV1,readNativeShardsV1,relieveNativeShardsV1} from "../🧱️native-shards/🟦️.ts";
import {nativeShardHoldV1} from "../🧱️native-shards/🔒️hold/🟦️.ts";''')
sub("🔌️real/🟦️.ts",'''["age-prune","log-rotate","staging-sweep"].includes(item.row.collector)''','''["age-prune","log-rotate","staging-sweep","native-shards"].includes(item.row.collector)''')
sub("🔌️real/🟦️.ts",'''.filter(item=>item.row.relief==="host"&&["age-prune"''','''.filter(item=>(item.row.relief==="host"||item.row.collector==="native-shards")&&["age-prune"''')
