"""🦀️ Run the authentic mounted neutral-boundary selector without altering compiler ingress."""
from pathlib import Path
import subprocess,json

ticket=Path(__file__).parent
root=ticket.parents[6]
artifact=root/"✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc"
command=["cargo","test","--manifest-path",str(artifact/"📦️packages/🦀️rust/Cargo.toml"),"--lib","shared_neutral_convex_profile_cases"]
result=subprocess.run(command,cwd=root,capture_output=True,text=True,encoding="utf8",errors="replace")
(ticket/"🗑️generated/convex-profile-native.txt").write_text("[DEBUG] Authentic mounted convex boundary selector\n"+result.stdout+result.stderr,encoding="utf8")
(ticket/"🗑️generated/convex-profile-native.json").write_text(json.dumps({"command":command,"code":result.returncode,"nativeTypeChecked":False,"nativeExecuted":False},ensure_ascii=False,indent=2)+"\n",encoding="utf8")
print("[DEBUG] "+json.dumps({"authenticCargoExit":result.returncode,"nativeTypeChecked":False,"nativeExecuted":False}))
