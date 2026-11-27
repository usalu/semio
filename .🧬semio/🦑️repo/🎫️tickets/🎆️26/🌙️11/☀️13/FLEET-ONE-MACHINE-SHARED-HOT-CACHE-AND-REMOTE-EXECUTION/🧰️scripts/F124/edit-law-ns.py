import os
p="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/👓️image-lease-readers/🟦️.ts"
s=open(p,encoding="utf8").read()
old="declare namespace ts{type Node=TsTypes.Node;type Expression=TsTypes.Expression;type SignatureDeclaration=TsTypes.SignatureDeclaration;"
assert s.count(old)==1
s=s.replace(old,"declare namespace ts{type Node=TsTypes.Node;type Expression=TsTypes.Expression;type Identifier=TsTypes.Identifier;type ArrowFunction=TsTypes.ArrowFunction;type FunctionExpression=TsTypes.FunctionExpression;type SignatureDeclaration=TsTypes.SignatureDeclaration;")
tmp=p+".f124tmp";open(tmp,"w",encoding="utf8").write(s);os.replace(tmp,p);print("ns ok")
