V = '️'
E = '✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine'


def mod(name, emoji, kebab, doc=None):
    d = dict(name=name, folder=emoji + V + kebab)
    if doc:
        d['doc'] = '//! %s%s %s' % (emoji, V, doc)
    return d


SPEC = dict(
    orig='video.rs.txt',
    file=E + '/🎥️video/🦀️.rs',
    nest=False,
    modules=[
        mod('cache', '💾', 'partial-movie-cache', 'Partial-movie cache of the headless video engine: the animation-hash keyed lookup with LRU eviction. See [`super`].'),
        mod('preview', '👀', 'live-preview', 'Live preview of the headless video engine: the preview outcome and the windowed, metadata-only and headless preview entry points. See [`super`].'),
        mod('render', '🏭', 'render-pipeline', 'Render pipeline of the headless video engine: the output formats and paths, scene capture and the frame recorder that renders a scene to disk. See [`super`].'),
        mod('renderer', '🎨', 'vello-renderer', 'Frame renderer of the headless video engine: the captured frames, the Vello renderer for native and wasip2 targets, the vector scene builder and the frame hashes. See [`super`].'),
        mod('scenes', '🎪', 'built-in-scenes', 'Built-in scenes of the headless video engine: the scenes resolved by content hash for presentation and video export. See [`super`].'),
        mod('writer', '✒', 'partial-movie-writer', 'Partial-movie writer of the headless video engine: the scene file writer, the sections subtitle writer and the partial-movie cache flush. See [`super`].'),
        mod('program', '🔧', 'video-render-program', 'Video render program of the headless video engine: the builder that turns captured frames into the host render program. See [`super`].'),
    ],
    external=[],
)
