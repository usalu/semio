"""🌿 Bind the shaded interior foliage to the actual battered core between the six clipped stools."""
import json
from pathlib import Path
t=Path(__file__).parent
p=next(Path.cwd().glob('*teaching/*architecture/*staffage/*catalog/*vegetation/hornbeam-hedge/*.json'))
for path in [p,t/'volumes/vegetation/hornbeam-hedge.json']:
    raw=path.read_text();v=raw.index('{',raw.index('"volumes":'));s=raw.index('{',raw.index('"500"',v))
    scale,n=json.JSONDecoder().raw_decode(raw[s:])
    core=next(k for k,x in enumerate(scale['thermal']) if x['id']=='th-core')
    belt=next((s for s in scale['body'] if s['id']=='hedge-core'),scale['thermal'][core]).copy()
    belt['id']='hedge-core';belt.pop('region',None)
    scale['body']=[belt]+[s for s in scale['body'] if s['id']!='hedge-core']
    shade=[[.40,-.15],[.40,.15],[.58,.20],[1.59,.165],[1.71,.083],[1.71,-.083],[1.59,-.165],[.58,-.20]]
    scale['thermal'][core]={**belt,'id':'th-core','from':.105,'to':2.895,'profile':shade,'region':'leaf-core'}
    scale['thermal']=[s for s in scale['thermal'] if s['id']!='th-interstool-face']+[dict(id='th-interstool-face',of='hedge-core',region='leaf-face')]
    scale['decisions'][0]=dict(en='Six clipped crowns follow their respective stools. The continuous shaded core tapers above the shoulder and ends below the outer leaf flush, leaving the foot of the hedge open around the six stems.',de='Sechs geschnittene Kronen folgen ihren jeweiligen Stöcken. Der durchgehende beschattete Kern verjüngt sich oberhalb der Schulter und endet unter dem äußeren Laubaustrieb. Der Fuß bleibt um die sechs Stämme offen.')
    scale['decisions'][1]=dict(en='Sun reaches the six crown panels and the continuous outer foliage between the stools. The recessed shaded volume ends below the cut crown shoulder. The six lower trunks retain their bark region and the transpiration patches follow each stool.',de='Licht erreicht die sechs Kronenteile und das durchgehende äußere Laub zwischen den Stöcken. Das zurückgesetzte Schattenvolumen endet unter der geschnittenen Kronenschulter. Die sechs unteren Stämme behalten ihre Rindenregion. Die Transpirationsflächen folgen dem jeweiligen Stock.')
    text=raw[:s]+json.dumps(scale,ensure_ascii=False,separators=(',',':'))+raw[s+n:]
    if raw!=path.read_text():raise RuntimeError(path)
    path.write_text(text)
