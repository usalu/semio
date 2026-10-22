import sys
sys.path.insert(0,".")
from anchored import patch
L="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/"
F=L+"⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/🏪️store/📇️format/🟦️.ts"
patch(F,[
('''import{repositoryCacheBudgetV1}from"../../../🟦️.ts";
''','''import{repositoryCacheBudgetV1}from"../../../🟦️.ts";
import{declareFormatStampV1,requireCurrentFormatStampsV1}from"../../../../../../../../🏗️build/🧷️format-stamps/🟨️.mjs";
'''),
('''const P=schema.$defs.Protocol.const;
''','''const P=schema.$defs.Protocol.const;
declareFormatStampV1("cache.graph-image",schema);
'''),
('''if(validateJsonSchemaSubset({$defs:schema.$defs,$ref:"#/$defs/Marker"},value.marker).length)throw Error("Current Graph record format required; run repo:cache-reset-graph");''','''if(validateJsonSchemaSubset({$defs:schema.$defs,$ref:"#/$defs/Marker"},value.marker).length){requireCurrentFormatStampsV1(["cache.graph-image"],"graph cache image","refuse");throw Error("Current Graph record format required; run repo:cache-reset-graph");}'''),
])
