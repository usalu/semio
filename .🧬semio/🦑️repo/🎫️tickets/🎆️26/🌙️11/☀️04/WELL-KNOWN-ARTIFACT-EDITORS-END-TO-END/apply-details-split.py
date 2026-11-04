import os
BASE = '/home/deck/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts'
SUBSETS = [
 '🎵️mp3/🏅️standards/🔖️mpeg1-layer3/🪆️subsets/✳️any',
 '☁️las/🏅️standards/🔖️1.0/🪆️subsets/🎩️header',
 '🎞️gif/🏅️standards/7️⃣87a/🪆️subsets/✳️any',
 '🎞️gif/🏅️standards/9️⃣89a/🪆️subsets/🧱️base',
 '🎥️mp4/🏅️standards/🔖️isobmff/🪆️subsets/✳️any',
 '🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/🧱️base',
 '🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/🔰️basic',
 '🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/🔬️tiny',
 '📼️avi/🏅️standards/🔖️1.0/🪆️subsets/🎛️hdrl',
 '📸️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/🧱️baseline',
 '📸️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/🧾️document',
 '🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any',
 '🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/🧱️baseline',
 '🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/🧾️document',
 '🔺️stl/🏅️standards/🔖️ascii/🪆️subsets/✳️any',
 '🧱️ply/🏅️standards/🔖️1.0/🪆️subsets/✳️any',
 '🖋️dxf/🏅️standards/🔖️r12/🪆️subsets/📰️header',
 '🖊️dwg/🏅️standards/4️⃣ac1018/🪆️subsets/✳️any',
 '🖊️dwg/🏅️standards/🔟ac1024/🪆️subsets/✳️any',
 '🗽️obj/🏅️standards/🔖️3.0/🪆️subsets/📐️geometry',
 '🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any',
 '📐️step/🏅️standards/🔖️ap214/🪆️subsets/🧱️base',
 '📐️step/🏅️standards/🔖️ap214/🪆️subsets/1️⃣cc1',
 '📐️step/🏅️standards/🔖️ap214/🪆️subsets/2️⃣cc2',
 '📐️step/🏅️standards/🔖️ap214/🪆️subsets/3️⃣cc3',
 '📐️step/🏅️standards/🔖️ap214/🪆️subsets/4️⃣cc4',
 '📐️step/🏅️standards/🔖️ap214/🪆️subsets/5️⃣cc5',
 '📐️step/🏅️standards/🔖️ap214/🪆️subsets/6️⃣cc6',
 '🏗️ifc/🏅️standards/4️⃣4/🪆️subsets/✳️any',
]
OLD = '.default_layout(edit::layout())'
NEW = '.default_layout(semio_s_artifact_stdio_contract::editing::snapshot_details_split_layout(main::WINDOW_KIND_ID, "Main"))'
for s in SUBSETS:
    p = os.path.join(BASE, s, '✏️editor', '🦀️.rs')
    text = open(p, encoding='utf8').read()
    n = text.count(OLD)
    if n == 1:
        open(p, 'w', encoding='utf8').write(text.replace(OLD, NEW))
        print('CHANGED', s)
    elif n == 0 and NEW in text:
        print('ALREADY', s)
    else:
        print('SKIP count=%d' % n, s)
