REPLACEMENTS = [
    ('(corner[0] - .55, corner[1] + .68, 0), 15, OPEN, 1.9)', '(corner[0] - .8, corner[1] + .84, 0), 15, OPEN, 1.9)'),
    ('(corner[0] - .55, corner[1] + .42, 0), 15, SHADED, 1.9)', '(corner[0] - .8, corner[1] + .58, 0), 15, SHADED, 1.9)'),
    ('key_x, key_y = self._at_plot(50, 0)[0] - .55, self._at_plot(50, 0)[1] + .16', 'key_x, key_y = self._at_plot(50, 0)[0] - .8, self._at_plot(50, 0)[1] + .32'),
    ('key_x, key_y = axes.c2p(.45, 500)[0], axes.c2p(.45, 500)[1] - .1', 'key_x, key_y = axes.c2p(.45, 500)[0], axes.c2p(.45, 500)[1] + .06'),
    ('(key_x, key_y - .25, 0), (key_x + .4, key_y - .25, 0), color=DATA', '(key_x, key_y - .22, 0), (key_x + .4, key_y - .22, 0), color=DATA'),
    ('(key_x + .5, key_y - .25, 0), 14, DATA, 2.4)', '(key_x + .5, key_y - .22, 0), 14, DATA, 2.4)'),
]
