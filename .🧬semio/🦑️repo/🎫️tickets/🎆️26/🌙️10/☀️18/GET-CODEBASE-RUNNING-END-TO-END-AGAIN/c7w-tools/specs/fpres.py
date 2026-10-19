from specs.lib import *
F = RW + '/🧊️renderer/🖥️frame-presentation/🦀️.rs'
c = Cut(F, 'fpres.rs').impl('impl AppPresenter {', [637, 1122], at=341)
c.child('🎞', 'present-state-and-engine-closers', 'present_state_and_engine_closers', [(4, 209), (1122, 1212)],
        'Present state and engine closers: the presenter state, the stall watch, the input wait, the resize and present phases and cursors, the presented retirement types and the engine surface and raster close delegates.')
c.child('🧯', 'retirement-and-present-steps', 'retirement_and_present_steps', [(211, 636)],
        'Retirement and present steps: the presented retirement, the surface resize steps, the presentation gate, the close steps and the admit and present steps.')
c.child('🎬', 'present-step-inner', 'present_step_inner_child', [(637, 1121)],
        'Present step inner: the long `AppPresenter::present_step_inner` that runs one presentation opportunity.')
SPEC = c.spec()
