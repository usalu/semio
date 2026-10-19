from specs.lib import *
F = W + '/🎟️prepared/🦀️.rs'
H = 'impl PreparedRenderJob {'
c = Cut(F, 'prep.rs').impl(H, [1616, 2028])
c.child('📊', 'credits-limits-and-fixed-lists', 'credits_limits_and_fixed_lists', [(13, 376)],
        'Credits, limits and fixed lists: the abandoned demands, the render limits and usage, the metadata and command page constants, the fixed page and list and the process permit.')
c.child('🧾', 'render-commands-and-packet', 'render_commands_and_packet', [(381, 838)],
        'Render commands and packet: the command kinds, the command pages and directories, the raster keep cursor and the prepared render packet with its drop.')
c.child('📮', 'render-input-and-mailbox', 'render_input_and_mailbox', [(840, 1330)],
        'Render input and mailbox: the prepared render input and its rejection, the mailbox slots and the receiver.')
c.child('🔧', 'render-job-state-and-close', 'render_job_state_and_close', [(1332, 1615)],
        'Render job state and close: the render job, the preparation sections, the draw measure cursor, the job rejection and the job constructors, close and abandonment steps.')
c.child('📏', 'draw-usage-measuring', 'draw_usage_measuring', [(1616, 2027)],
        'Draw usage measuring: the draw usage census and the layer, scene pass, shadow and material cursor steps.')
c.child('🏁', 'job-completion-and-interactive-job', 'job_completion_and_interactive_job', [(2028, 2414)],
        'Job completion and interactive job: the measuring, metadata and pipeline steps, the job completion, its drop and the `InteractiveJob` implementation.')
c.child('🛡', 'presentation-gate', 'presentation_gate', [(2417, 2611)],
        'Presentation gate: the render rejection, the presenter witness, the replacement and pending packets, the gate and the present tokens.')
SPEC = c.spec()
