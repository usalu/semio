"""📐️ Check real bilingual annotated orthographics after complete-ink placement."""
import importlib
from pathlib import Path
import numpy as np

V=importlib.import_module('🎓️teaching.🏛️architecture.🏢️nakagin.🎨️visuals.🐍️')
V._engine.config.pixel_width,V._engine.config.pixel_height=1920,1080
V._engine.config.frame_width,V._engine.config.frame_height=16,9
output=Path(__file__).resolve().parent/'🗑️generated'

class Sheet(V.NakaginDrawing,V._d.Drafting):
    def number(self,value,digits=2):
        return f'{value:.{digits}f}'.replace('.',',' if self.language=='de' else '.')

for language in ('en','de'):
    sheet=Sheet()
    sheet.language=language
    drawing=sheet.capsule_views(V._model.capsule_type('A'),scale=.65)
    front,side,plan=(drawing.parts[name] for name in ('front','side','plan'))
    assert side.get_left()[0]-front.get_right()[0]>=1-1e-9
    assert plan.get_bottom()[1]-max(front.get_top()[1],side.get_top()[1])>=1-1e-9
    shapes=[shape for shape in side if hasattr(shape,'world_vertices')]
    assert shapes
    for shape in shapes[:16]:
        projected=V._screen(side.view,shape.world_vertices)
        actual=shape.get_all_points()
        np.testing.assert_allclose((actual.min(axis=0),actual.max(axis=0)),(projected.min(axis=0),projected.max(axis=0)),atol=1e-9,rtol=0)
    camera=V._engine.Camera(background_color=V.BACKGROUND)
    camera.capture_mobjects([drawing])
    camera.get_image().save(output/f'annotation-layout-{language}.png')
    print('[DEBUG] Real annotated orthographic clearance and projection verified: '+language,flush=True)
