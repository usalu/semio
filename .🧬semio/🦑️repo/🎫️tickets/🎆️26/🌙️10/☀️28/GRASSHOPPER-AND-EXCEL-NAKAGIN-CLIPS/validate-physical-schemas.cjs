/** 🧬 Run strict source schema and portable physical contract validators. */
const path=require('node:path');
const root=path.resolve(__dirname,'../../../../../../..');
const base=path.join(root,'🎓️teaching/🏛️architecture/🏢️nakagin/🧪️tests');
for(const domain of ['', '📦️casing','🧱️panel','🪟️unit','🔧️mounts','🧭️poses']){
  console.log('[DEBUG] strict physical schema '+(domain||'shared'));
  require(path.join(base,domain,'🟨️.cjs'));
}
