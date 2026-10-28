REPLACEMENTS = [
    ('height=AXES_HEIGHT, x_digits=x_digits, x_ticks=list(x_ticks), y_ticks=list(range(0, y_max + 1, y_step)))', 'height=AXES_HEIGHT, x_digits=x_digits, x_ticks=list(x_ticks), y_ticks=list(range(0, y_max + 1, y_step)), corner=True)'),
    ('        key = self._at("┄ " + self.tr("if the lines were parallel", "wenn die Linien parallel wären"), (self._at_plot(10, YMAX)[0] + .2, self._at_plot(10, YMAX)[1] - .7, 0), 15, OPEN, 2.9)\n',
     '        key_x, key_y = self._at_plot(10, YMAX)[0] + .2, self._at_plot(10, YMAX)[1] - .7\n'
     '        key = _mn.VGroup(_room.dashed((key_x, key_y, 0), (key_x + .4, key_y, 0), OPEN, 2.4), self._at(self.tr("if the lines were parallel", "wenn die Linien parallel wären"), (key_x + .5, key_y, 0), 15, OPEN, 2.9))\n'),
]
