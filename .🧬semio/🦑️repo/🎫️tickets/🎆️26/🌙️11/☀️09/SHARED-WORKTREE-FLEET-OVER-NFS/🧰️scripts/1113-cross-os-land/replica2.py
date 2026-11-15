from lib import *
def command(t):
    return sub(t,' for(const item of report.differing)say("fleet.differs",',' for(const item of report.warnings)say(`fleet.warning.${item.kind}`,{origin:item.originId,detail:item.detail});\n for(const item of report.differing)say("fleet.differs",')
edit("🔁️replica/⌨️command/🟦️.ts",command)
def texts(t):
    anchor='''    "fleet.differs": {
      "en": "  {origin} differs in {field}: {value} instead of {expected}",
      "de": "  {origin} weicht in {field} ab: {value} statt {expected}"
    },
'''
    block='''    "fleet.warning.git-version": {
      "en": "  warning: {origin} runs git {detail}, other machines run another version",
      "de": "  Warnung: {origin} verwendet git {detail}, andere Maschinen eine andere Version"
    },
    "fleet.warning.volume": {
      "en": "  warning: the volume of {origin} behaves differently from most ({detail}); the view is pinned, so this is a capability difference, not a divergence",
      "de": "  Warnung: Das Volume von {origin} verhält sich anders als die meisten ({detail}); die Sicht ist gepinnt, es ist ein Unterschied der Fähigkeit, keine Abweichung"
    },
    "fleet.warning.over-budget": {
      "en": "  warning: {origin} has {detail} tracked paths longer than its operating system holds below its worktree",
      "de": "  Warnung: {origin} hat {detail} versionierte Pfade, die länger sind, als sein Betriebssystem unter dem Arbeitsverzeichnis zulässt"
    },
    "fleet.warning.symlinks": {
      "en": "  warning: {origin} cannot create symbolic links but the repository tracks {detail}",
      "de": "  Warnung: {origin} kann keine symbolischen Links anlegen, das Repository versioniert aber {detail}"
    },
'''
    return sub(t,anchor,anchor+block)
edit("🔁️replica/📖️texts/🔣️.json",texts)
