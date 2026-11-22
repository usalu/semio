import json,sys
data=json.load(open(sys.argv[1]))
def hexrgb(h): h=h.lstrip('#'); return "[%d, %d, %d]"%(int(h[0:2],16),int(h[2:4],16),int(h[4:6],16))
names={"default":"DEFAULT","pointer":"POINTER","grab":"GRAB","grabbing":"GRABBING","crosshair":"CROSSHAIR","move":"MOVE"}
variants={"default":"Default","pointer":"Pointer","grab":"Grab","grabbing":"Grabbing","crosshair":"Crosshair","move":"Move"}
out=['//! 👻️ Tutorial ghost cursor glyphs: the Semio cursor assets (`🖼️assets/👆️cursor`) as filled polygons in their own viewBox space, one table per appearance, generated from `🖼️assets/👆️cursor/👻️ghost-glyphs/🔣️.json` — React paints the same assets through `--cursor-ghost-<glyph>`.','','use semio_framework::onboarding::IntroductionCursor;','','/// 🔷️ One filled and outlined polygon of a ghost cursor glyph; `fill` and `stroke` are sRGB bytes, `points` are in viewBox pixels.','pub struct GhostGlyphPolygon {','    pub fill: [u8; 3],','    pub stroke: [u8; 3],','    pub stroke_width: f32,','    pub points: &\'static [[f32; 2]],','}','','/// 🖱️ One ghost cursor glyph: its viewBox size and polygons in paint order.','pub struct GhostGlyph {','    pub width: f32,','    pub height: f32,','    pub polygons: &\'static [GhostGlyphPolygon],','}','']
for a in ("light","dark"):
    for g,v in data["appearances"][a].items():
        out.append("static %s_%s: GhostGlyph = GhostGlyph { width: %s, height: %s, polygons: &["%(a.upper(),names[g],v["width"],v["height"]))
        for p in v["polygons"]:
            pts=", ".join("[%s, %s]"%(x,y) for x,y in p["points"])
            out.append("    GhostGlyphPolygon { fill: %s, stroke: %s, stroke_width: %s, points: &[%s] },"%(hexrgb(p["fill"]),hexrgb(p["stroke"]),p["strokeWidth"],pts))
        out.append("] };")
        out.append("")
out.append("/// 🖱️ The glyph of `cursor` for the light or dark appearance, matching React `--cursor-ghost-<glyph>`.")
out.append("pub fn tutorial_ghost_glyph(cursor: IntroductionCursor, dark: bool) -> &'static GhostGlyph {")
out.append("    match (cursor, dark) {")
for g,v in variants.items():
    out.append("        (IntroductionCursor::%s, false) => &LIGHT_%s,"%(v,names[g]))
    out.append("        (IntroductionCursor::%s, true) => &DARK_%s,"%(v,names[g]))
out.append("    }")
out.append("}")
open(sys.argv[2],"w").write("\n".join(out)+"\n")
