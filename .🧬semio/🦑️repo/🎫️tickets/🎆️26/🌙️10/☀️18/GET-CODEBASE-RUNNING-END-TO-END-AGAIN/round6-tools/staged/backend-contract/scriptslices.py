"""Line-oriented slices of the live neutral CLI script that hold the per-backend code, with the hashes they had when the staged adapters were cut. Refuses when the file drifted."""
import hashlib,re
class Drift(Exception):pass
def find(lines,prefix,start=0):
    hits=[i for i in range(start,len(lines)) if lines[i].startswith(prefix)]
    if not hits:raise Drift(f"anchor missing: {prefix[:70]!r}")
    return hits[0]
def slices(text):
    lines=text.split("\n")
    tools=find(lines,'    if(command==="tools"){');reset=find(lines,'    if(command==="reset"){',tools);cache=find(lines,'    if(["graph","show"].includes(command)){',reset)
    setup_b=find(lines,'      if(tools!=="nx"&&tools!=="toolchains"){',tools);setup_n=find(lines,'      if(tools!=="bazel"&&tools!=="toolchains"){',setup_b)
    nx=find(lines,'    if(selected.tool==="nx"){');inputs=find(lines,'    if(command==="inputs")throw',nx);end=find(lines,'    },additionalBytes=>prepareBuildToolStorageV1(',inputs)
    return {
     "tools":(tools,reset),"tools_bazel":(setup_b+1,setup_n-1),"tools_nx":(setup_n,setup_n+1),
     "reset":(reset,cache),"nx":(nx,inputs),"bazel":(inputs,end),
     "lines":lines}
def part(s,name):
    a,b=s[name];return "\n".join(s["lines"][a:b])
def digest(text):return hashlib.sha256(text.encode()).hexdigest()
