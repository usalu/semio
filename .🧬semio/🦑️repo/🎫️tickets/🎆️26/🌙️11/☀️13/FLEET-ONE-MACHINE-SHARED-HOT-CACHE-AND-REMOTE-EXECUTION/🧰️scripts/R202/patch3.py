import sys
K=sys.argv[1];OUT=sys.argv[2];STAGE=sys.argv[3]
def rd(p): return open(p,encoding='utf8').read()
def wr(n,t): open(OUT+'/'+n,'w',encoding='utf8').write(t)
def sub(t,old,new,count=1):
    assert t.count(old)==count,(old[:70],t.count(old))
    return t.replace(old,new)
l=rd(K+'/look.base3.mjs')
l=sub(l,'''/** 🔭️ The batches of the lanes of a take pass in the order the farm serves them: one batch of each lane in turn, so the owners of a short lane never wait behind thousands of the long one. Every name is in exactly one batch, in its lane's order.
 * @see ./🧪️tests/🟦️.ts */
export function lookBatchesV1(lanes, size) {
  const rest = lanes.map(lane => [...lane]), batches = [];
  for (let live = rest.filter(lane => lane.length > 0); live.length > 0; live = live.filter(lane => lane.length > 0)) for (const lane of live) batches.push(lane.splice(0, size));
  return batches;
}''','''/** 🔭️ The batches of the lanes of a take pass in the order the farm serves them: one batch of each lane in turn, so the owners of a short lane never wait behind thousands of the long one; an owner that `isolated` names (its look computes the assembly authority of a native or release owner, minutes under load while a plain owner takes seconds) is a batch of its own, because a job answers only when its slowest owner is done. Every name is in exactly one batch, in its lane's order.
 * @see ./🧪️tests/🧱️isolated */
export function lookBatchesV1(lanes, size, isolated = () => false) {
  const rest = lanes.map(lane => [...lane]), batches = [];
  for (let live = rest.filter(lane => lane.length > 0); live.length > 0; live = live.filter(lane => lane.length > 0)) for (const lane of live) {
    const batch = [];
    while (lane.length > 0 && batch.length < size) {
      if (isolated(lane[0])) { if (batch.length === 0) batch.push(lane.shift()); break; }
      batch.push(lane.shift());
    }
    batches.push(batch);
  }
  return batches;
}''')
l=sub(l,'    start(lanes) {','    start(lanes, isolated) {')
l=sub(l,'const batches = lookBatchesV1(lanes, size);','const batches = lookBatchesV1(lanes, size, isolated);')
wr('look.new3.mjs',l)
d=rd(K+'/look.base3.d.mts')
d=sub(d,'export function lookBatchesV1(lanes: readonly (readonly string[])[], size: number): string[][];','export function lookBatchesV1(lanes: readonly (readonly string[])[], size: number, isolated?: (name: string) => boolean): string[][];')
d=sub(d,'start(lanes: readonly (readonly string[])[]): void;','start(lanes: readonly (readonly string[])[], isolated?: (name: string) => boolean): void;')
wr('look.new3.d.mts',d)
p=rd(STAGE+'/pn.new2.mjs')
p=sub(p,'lookQueue?.start([...lanes,split.heavy]);','lookQueue?.start([...lanes,split.heavy],name=>{const owner=byName.get(name);return owner!==undefined&&(existsSync(join(workspaceRoot,owner.json.metadata?.nativeRoot??owner.json.root,"Cargo.toml"))||Object.values(owner.json.targets??{}).some(target=>target.metadata?.semio?.release));});')
wr('pn.new3.mjs',p)
print('ok')
