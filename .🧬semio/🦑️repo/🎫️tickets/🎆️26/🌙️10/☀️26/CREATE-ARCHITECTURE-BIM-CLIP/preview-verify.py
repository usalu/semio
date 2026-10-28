from pathlib import Path
import json
import av
ticket=Path(__file__).resolve().parent
work=ticket/"🗑️generated"/"preview"
videos=list((work/"de"/"speaker"/"rendered").glob("BIM_information*.mp4"))
assert len(videos)==1, videos
video=videos[0]
with av.open(str(video)) as container:
    stream=container.streams.video[0]
    assert (stream.width,stream.height)==(1280,720)
    assert float(stream.average_rate)==24
    count=0
    poster=None
    for frame in container.decode(video=0):
        if frame.pts is not None and float(frame.pts*stream.time_base)>=30 and poster is None:
            poster=frame.to_image()
        count+=1
    assert count>1000
    assert container.streams.audio
    duration=float(container.duration/av.time_base)
    result=dict(path=str(video),width=stream.width,height=stream.height,frames=count,fps=float(stream.average_rate),duration=duration,bytes=video.stat().st_size)
    assert poster is not None
    poster.save(work/"poster.png")
    (work/"verification.json").write_text(json.dumps(result,indent=2),encoding="utf8")
    print(json.dumps(result))
