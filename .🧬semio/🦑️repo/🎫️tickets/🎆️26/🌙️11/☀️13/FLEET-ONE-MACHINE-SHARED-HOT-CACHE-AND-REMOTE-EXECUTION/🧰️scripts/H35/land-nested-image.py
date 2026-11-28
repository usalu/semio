import sys,os;sys.path.insert(0,os.path.dirname(__file__))
from lib import edit
edit("🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧹️cleanup/🎛️command/🟦️.ts",[
('if(outer)throw Error("Nested persistent image requires actual parent scope: "+options.directory+" lies inside "+outer.image!.root+" ("+outer.stage+")");}',
 'if(outer){const leased=outer.stage==="ready"&&outer.guardOwner!==undefined&&(()=>{try{requireIssuedResourceControlRefinementV1(outer.options.control,options.control);return true;}catch{return false;}})();if(!leased)throw Error("Nested persistent image requires actual parent scope: "+options.directory+" lies inside "+outer.image!.root+" ("+outer.stage+"); the parent must be mounted and leased by the same task / Verschachteltes persistentes Abbild braucht den tatsächlichen Elternbereich: "+options.directory+" liegt in "+outer.image!.root+" ("+outer.stage+"); das Elternabbild muss eingehängt und von derselben Aufgabe belegt sein");}}'),
])
