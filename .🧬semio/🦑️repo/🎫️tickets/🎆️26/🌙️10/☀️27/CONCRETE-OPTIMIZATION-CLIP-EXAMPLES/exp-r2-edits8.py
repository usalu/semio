REPLACEMENTS = [
    ('(corner[0] + .35, corner[1] + .58, 0), 15, OPEN, 1.9)', '(corner[0] - .55, corner[1] + .68, 0), 15, OPEN, 1.9)'),
    ('(corner[0] + .35, corner[1] + .36, 0), 15, SHADED, 1.9)', '(corner[0] - .55, corner[1] + .42, 0), 15, SHADED, 1.9)'),
    ('key_x, key_y = self._at_plot(50, 0)[0] + .35, self._at_plot(50, 0)[1] + .14', 'key_x, key_y = self._at_plot(50, 0)[0] - .55, self._at_plot(50, 0)[1] + .16'),
    ('key_x, key_y = axes.c2p(.55, 500)[0], axes.c2p(.55, 500)[1] - .12', 'key_x, key_y = axes.c2p(.45, 500)[0], axes.c2p(.45, 500)[1] - .1'),
    ('(key_x, key_y - .3, 0), (key_x + .4, key_y - .3, 0), color=DATA', '(key_x, key_y - .25, 0), (key_x + .4, key_y - .25, 0), color=DATA'),
    ('self._at(self.tr("simulated rooms, 50 % window", "simulierte Räume, 50 % Fenster"), (key_x + .5, key_y - .3, 0), 14, DATA, 2.6)', 'self._at(self.tr("simulated, 50 % window", "simuliert, 50 % Fenster"), (key_x + .5, key_y - .25, 0), 14, DATA, 2.4)'),
    ('frame = self._rect((frame_x, 2.5 - .5 * 2.6, 0), width + .06, .5 * 5.1, ALERT, stroke=3)', 'frame = self._rect((frame_x, 1.3, 0), width + .06, 2.7, ALERT, stroke=3)'),
]
