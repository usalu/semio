"""✅ Verify the final actual-UI review film and retain bounded visual evidence."""
import hashlib,importlib,json,os,sys,time
from pathlib import Path
import av
import numpy as np
from PIL import Image
root=next(path for path in Path(__file__).resolve().parents if (path/'🧰️framework').is_dir())
sys.path.insert(0,str(root))
runtime=importlib.import_module('🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.⚙️runtime.🐍️')
ticket=Path(__file__).parent
output=Path(os.environ['SEMIO_TEACHING_OUTPUT_ROOT'])


def frames(path):
    """🎞️ Decode complete video streams independently of the UI renderer."""
    with av.open(str(path)) as container:
        stream=container.streams.video[0]
        result=[frame.to_ndarray(format='rgb24') for frame in container.decode(stream)]
        return result,float(stream.average_rate)


movie=ticket/'exact-ui.mp4'
film,fps=frames(movie)
tour,_=frames(output/'tour.mp4')
lesson,_=frames(output/'lesson.mp4')
body,_=frames(output/'body'/'v'/'device.mp4')
assert fps==8 and film[0].shape==(900,1600,3)
assert len(film)==len(tour)+len(lesson)==99
assert len(lesson)==len(body)+32==70
assert np.array_equal(film[0],film[-1])
assert np.array_equal(film[len(tour)-1],film[len(tour)])
trace_path=next((output/'trace').rglob('👆️.json'))
trace=json.loads(trace_path.read_text(encoding='utf8'))
events=trace['events']
press=next(event['time'] for event in events if event['type']=='press')
release=next(event['time'] for event in events if event['type']=='release')
click=next(event['time'] for event in events if event['type']=='click')
assert press<release==click
x1,x2=round((540.8-6.4)*1588/1587.2),round((700.8-6.4)*1588/1587.2)
y1,y2=round((403.2-57.6)*808/807.2),round((563.2-57.6)*808/807.2)
heat=[]
for index,image in enumerate(body):
    tile=image[y1:y2,x1:x2].astype(float)
    red=(tile[:,:,0]>120)&(tile[:,:,0]>tile[:,:,1]*1.5)&(tile[:,:,0]>tile[:,:,2]*1.5)
    heat.append(int(red.sum()))
assert all(value==0 for index,value in enumerate(heat) if index/fps<click)
assert max(value for index,value in enumerate(heat) if index/fps>=click)>100
for name,index in [('exact-ui-click.png',len(tour)+4),('exact-ui-device.png',len(tour)+16+round((click+.8)*fps)),('exact-ui-menu.png',12)]:
    Image.fromarray(film[index]).save(ticket/name)
runtime.run_command([*runtime.ffmpeg_command(),'-hide_banner','-loglevel','error','-xerror','-i',str(movie),'-f','null','-'],log=output/f'verify-{time.time_ns()}.log')
sha=hashlib.sha256(movie.read_bytes()).hexdigest()
endpoint=hashlib.sha256(film[0].tobytes()).hexdigest()
report=f'''# Actual UI Review Film

The retained [exact-ui.mp4](./exact-ui.mp4) is rendered with the actual semio React WindowChrome, Navbar, Footer, Popover and Tree components, source CSS, bundled fonts, source SVG icons and real computed cursor assets. A canonical1600×900 CSS viewport is rasterized at the requested device scale. The popup uses the real portal boundary; its hover leaves siblings unchanged.

The final film contains **{len(film)} frames at8fps,12.375seconds,1600×900**. The29-frame navigation tour joins a70-frame focused lesson, preserving all38native scientific source frames with16-frame transition pads. First/last decoded overview RGB pixels and the tour/lesson handoff are byte-identical. Release and click are simultaneous at{click:.3f}s. An independent red-pixel oracle proves the heater remains off before the committed click and emits heat afterward. FFmpeg independently decodes the entire H.264 film without errors.

Movie SHA-256:{sha}

Decoded overview RGB SHA-256:{endpoint}

This is a focused interaction/layout review film. The complete curriculum has not been regenerated. Allfive production lesson envelopes independently pass encoded frame-order, audio and shared-boundary regression checks; see[exact-ui-tests.md](./exact-ui-tests.md). The authored scientific body uses the native geometry/typography port; the application chrome uses actual browser UI components.
'''
(ticket/'exact-ui-render.md').write_text(report,encoding='utf8')
if '--no-open' not in sys.argv:
    if sys.platform=='win32':os.startfile(movie)
    elif sys.platform=='darwin':runtime.run_command(['open',str(movie)])
    else:runtime.run_command(['xdg-open',str(movie)])
print(f'[DEBUG] Actual UI movie verified:{len(film)}frames,12.375s,exact endpoints and click-gated heat',flush=True)
