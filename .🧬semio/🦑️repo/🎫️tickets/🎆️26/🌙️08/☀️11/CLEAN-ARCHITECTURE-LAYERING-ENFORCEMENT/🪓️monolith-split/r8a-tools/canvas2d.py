import sys,os,re
E=sys.argv[1]
ed=E+'/🎭️modes/✏️edit/🦀️.rs'
t=open(ed,encoding='utf8').read()
i=t.index('pub const PUZZLE2D_PLAY_MODE_EDIT')
header=t[:i]
j=t.index('//#region 🔖️PaneCamera')
head=t[:j]; tail=t[j:]
doc_end=header.index('\n\n')
docs=header[:doc_end]; imports=header[doc_end:]
canvas_doc='//! 🖼️ Puzzle 2d `edit` mode canvas: the pane-camera framing, board-scene and engagement-HUD builders the three windows share. Each window binds these to its own pane; nothing here is pane-specific.'
tail=tail.replace('detail::ZOOM_SCALE','crate::editor::puzzle2d::config::PUZZLE2D_DETAIL_ZOOM_SCALE').replace('selection::ZOOM_SCALE','crate::editor::puzzle2d::config::PUZZLE2D_SELECTION_ZOOM_SCALE').replace('overview::ZOOM_SCALE','crate::editor::puzzle2d::config::PUZZLE2D_OVERVIEW_ZOOM_SCALE')
os.makedirs(E+'/🎭️modes/✏️edit/🖼️canvas',exist_ok=True)
open(E+'/🎭️modes/✏️edit/🖼️canvas/🦀️.rs','w',encoding='utf8').write(canvas_doc+'\n'+imports.rstrip('\n')+'\n\n'+tail)
edit_new=head.rstrip('\n')+'\n\n#[path = "🖼️canvas/🦀️.rs"]\npub mod canvas;\n'
open(ed,'w',encoding='utf8').write(edit_new)
cfg=E+'/🎚️config/🦀️.rs'; c=open(cfg,encoding='utf8').read()
c=c.rstrip('\n')+'\n\n/// 🔭️ Zoom factor each triptych pane applies to the framed base zoom.\npub const PUZZLE2D_OVERVIEW_ZOOM_SCALE: f64 = 0.68;\npub const PUZZLE2D_DETAIL_ZOOM_SCALE: f64 = 2.15;\npub const PUZZLE2D_SELECTION_ZOOM_SCALE: f64 = 0.36;\n'
open(cfg,'w',encoding='utf8').write(c)
for w,k,v in (('👁️overview','OVERVIEW','0.68'),('🔍️detail','DETAIL','2.15'),('🎯️selection','SELECTION','0.36')):
    p=E+'/🎭️modes/✏️edit/🪟️windows/'+w+'/🦀️.rs'; x=open(p,encoding='utf8').read()
    old='pub const ZOOM_SCALE: f64 = %s;'%v; assert old in x,(w,)
    x=x.replace(old,'pub const ZOOM_SCALE: f64 = crate::editor::puzzle2d::config::PUZZLE2D_%s_ZOOM_SCALE;'%k)
    x=x.replace('edit::render_canvas','edit::canvas::render_canvas').replace('edit::puzzle2d_engagement','edit::canvas::puzzle2d_engagement')
    open(p,'w',encoding='utf8').write(x)
hub=E+'/🦀️.rs'; h=open(hub,encoding='utf8').read()
h=h.replace('edit::puzzle2d_engagement','edit::canvas::puzzle2d_engagement'); open(hub,'w',encoding='utf8').write(h)
ut=E+'/🧪️tests/🔬️unit/🦀️.rs'; u=open(ut,encoding='utf8').read(); u=u.replace('edit::puzzle2d_board_scene','edit::canvas::puzzle2d_board_scene'); open(ut,'w',encoding='utf8').write(u)
for p in (E+'/🎭️modes/✏️edit/☑️options/🖌️brush/🧪️tests/🔬️unit/🦀️.rs',E+'/🎭️modes/✏️edit/🛠️tools/🪣️fill/🧪️tests/🔬️unit/🦀️.rs'):
    x=open(p,encoding='utf8').read(); x=x.replace('modes::edit::puzzle2d_engagement','modes::edit::canvas::puzzle2d_engagement'); open(p,'w',encoding='utf8').write(x)
print('ok')
