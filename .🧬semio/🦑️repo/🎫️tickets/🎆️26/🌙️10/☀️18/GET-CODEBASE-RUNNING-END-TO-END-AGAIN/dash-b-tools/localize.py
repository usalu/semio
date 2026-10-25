import re,sys
exec(open('/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/dash-b-tools/sites.py').read().split("for n,(f,p,a)")[0])
DE={
0:'"en und de brauchen einzeiligen Text aus Nicht-Leerzeichen mit höchstens 1024 Zeichen und ohne Terminal-Steuerzeichen"',
2:'format!("{what} {value:?} ist ein absoluter Pfad oder enthält ein NUL; schreibe `{{workspace}}/…`")',
3:'format!("Umgebungsname {name:?} ist kein Variablenname")',
4:'format!("{name} gehört zur Laufrichtlinie des Dashboards und wird nie deklariert")',
5:'format!("buildFlags-Eintrag {flag:?} ist kein Flag")',
6:'format!("mcp-Server {:?} muss aus kleingeschriebenen Wörtern bestehen, verbunden durch `-`, `.` oder `_`", mcp.server)',
7:'"mcp braucht mindestens einen Client"',
8:'format!("mcp-Client {client:?} ist keiner von {}", AGENT_CLIENTS.join(", "))',
9:'format!("mcp-Client {client} nennt {name:?}, das das Werkzeug nicht annimmt")',
10:'format!("Verb {verb:?} muss ein kleingeschriebenes Wort sein")',
11:'format!("ID {id:?} muss aus kleingeschriebenen Wörtern bestehen, verbunden durch `-`, `.` oder `_`")',
12:'format!("{what} {reference:?} muss `<project>:<target>[:configuration]`, `playground:<variant>`, `tool:<project>/<id>` oder `ticket:<ticket id>/<id>` sein")',
13:'format!("{what} {reference:?} ist doppelt aufgeführt")',
14:'format!("{what} {:?} ist doppelt aufgeführt", declaration.run)',
15:'format!("Parametername {key:?} muss eine Parameter-ID sein")',
16:'format!("Parameter {key:?} muss Text, eine Zahl oder ein Wahrheitswert sein")',
17:'"braucht `port` oder `portEnv`"',
18:'"`port` muss zwischen 1 und 65535 liegen"',
19:'format!("portEnv {name:?} nennt weder eine Variable noch einen Parameter")',
20:'format!("Pfad {path:?} muss mit `/`, `?` oder `#` beginnen und darf keine Leerzeichen enthalten")',
21:'"`printed` nimmt die ganze ausgegebene Adresse und lässt keinen Platz für `path`"',
22:'format!("Parameter-ID {:?} ist ein eingebautes Token", declaration.id)',
23:'format!("Parameter {:?} ist doppelt deklariert", declaration.id)',
24:'"`appliesTo` gehört zu den globalen Achsen des Wurzelprojekts des Arbeitsbereichs"',
25:'"`suggestions` gehört zu Textparametern"',
26:'"Projektvorschläge haben keine verwandten Felder"',
27:'format!("Vorschlagsfeld {related:?} muss einen anderen Textparameter nennen")',
28:'"`appliesTo.all` muss true sein"',
29:'"`appliesTo` kann `all` und `verbs` nicht verbinden"',
30:'"`appliesTo` wählt nichts aus"',
31:'format!("Verb {verb:?} muss ein kleingeschriebenes Wort sein")',
32:'"`appliesTo.verbs` nennt kein Verb"',
33:'format!("{kind} hat kein `{name}`")',
34:'"eine Auswahl braucht `values`"',
35:'format!("Wert-ID {:?} muss ein Wort ohne `=` sein", value.id)',
36:'format!("Wert {:?} ist doppelt deklariert", value.id)',
37:'format!("Standardwert {other} gehört nicht zu den Werten")',
38:'"ein Flag braucht `env`, `buildFlags` oder `args`"',
39:'format!("Standardwert {other} muss `true` oder `false` sein")',
40:'"ein Text geht an genau eines von `valueEnv`, `valueFlag` oder `valuePositional`"',
41:'"`valuePositional` ist `true` oder fehlt"',
42:'format!("valueEnv {name:?} ist kein Variablenname")',
43:'format!("{name} gehört zur Laufrichtlinie des Dashboards und wird nie deklariert")',
44:'format!("valueFlag {flag:?} ist kein Flag")',
45:'format!("Standardwert {other} muss Text sein")',
46:'"`parameters` sind die globalen Achsen und gehören zum Wurzelprojekt des Arbeitsbereichs"',
47:'"eine Gruppe braucht `targets`"',
48:'format!("Target {name:?} muss ein einzelner Target-Name sein, einmal aufgeführt")',
49:'"eine Gruppe braucht `projects`"',
50:'format!("Projekt {name:?} muss ein einzelner Projektname sein, einmal aufgeführt")',
51:'"ein Werkzeug braucht ein `command`"',
52:'format!("cwd {:?} muss relativ zum Arbeitsbereich sein", tool.cwd)',
53:'"ein Verbund braucht `members`"',
54:'format!("Mitglied {:?} nennt keinen Befehl", member.run)',
55:'format!("Mitglied {:?} ist ein Verbund; Verbünde lassen sich nicht schachteln", member.run)',
56:'format!("Mitglied {:?} hat keinen Parameter {key:?}", member.run)',
57:'format!("Umgebungs-Token `{{{token}}}` von Mitglied {:?} ist nicht `{{workspace}}`", member.run)',
58:'format!("Projekt {pattern:?} nennt kein Projekt")',
59:'format!("Projekt {pattern:?} hat kein Target {}", targets.join(" oder "))',
60:'format!("requires {:?} nennt keinen Befehl", need.run)',
61:'format!("requires {:?} hat keinen Parameter {key:?}", need.run)',
62:'format!("Umgebungs-Token `{{{token}}}` von requires {:?} ist nicht `{{workspace}}`", need.run)',
63:'"`{port}` braucht eine `ready`-Deklaration"',
64:'format!("Token `{{{token}}}` nennt keinen Parameter dieses Befehls")',
65:'format!("Token `{{{token}}}` in einem Standardwert nennt keinen Parameter dieses Befehls")',
66:'format!("Textparameter {:?} führt ins Leere: `valueEnv`, `valueFlag`, `valuePositional` deklarieren oder `{{{}}}` verwenden", parameter.id, parameter.id)',
67:'format!("ready.portEnv {name:?} nennt keinen Parameter und keine deklarierte Variable, und es ist kein `port` angegeben")',
69:'format!("aufgelöste Umgebung enthält {key}")',
70:'format!("Renderer {renderer} löst zum Build-Target {id} auf, das der Projektgraph nicht hat")',
71:'format!("Bezeichnung ist {} Zeichen lang", entry.label.chars().count())',
}
edits={}
for n,(f,p,a) in enumerate(sites):
    if n==1: continue
    short=f.split('/')[1][:3]
    if n==68: new='LocalizedText::of(error.clone(), error)'; newargs=new
    elif short=='🧶️' and a.startswith('entry.id.clone(), '):
        en=a[len('entry.id.clone(), '):]
        if en.endswith('.into()'): en=en[:-len('.into()')]
        newargs='entry.id.clone(), LocalizedText::of(%s, %s)'%(en,DE[n])
    elif short=='🩺️c': newargs='LocalizedText::of(%s, %s)'%(a,DE[n])
    else: newargs='%s, %s'%(a,DE[n])
    edits.setdefault(f,[]).append((p,a,newargs))
for f,rows in edits.items():
    s=open(D+f).read()
    for p,a,newargs in sorted(rows,reverse=True):
        o=s.index('(',p); e=balanced(s,o)
        assert s[o+1:e]==a,(f,p)
        s=s[:o+1]+newargs+s[e:]
    open(D+f,'w').write(s)
print('applied',sum(len(v) for v in edits.values()))
