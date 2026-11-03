p='🐍️.py'
s=open(p,encoding='utf-8',newline='').read()
crlf='\r\n' in s
s=s.replace('\r\n','\n')
a=s.index("def _rack(")
b=s.index("class Beat3_GeraetePhiE")
s=s[:a]+s[b:]
a=s.index('        seated = seated_person_glyph(np.array([room["x_l"] + 1.4, y_f, 0.0])')
b=s.index('        warm = [laptop.get_center(), server_box.get_top() + UP * 0.04]')
new=open(__file__.replace('m1_patch2.py','m1_b3.txt'),encoding='utf-8').read()
s=s[:a]+new+s[b:]
s=s.replace('        warm = [laptop.get_center(), server_box.get_top() + UP * 0.04]','        warm = [laptop.get_center(), server.get_top() + DOWN * 0.1]')
s=s.replace('self.play(FadeIn(room["shell"]), FadeIn(room["glass"]), FadeIn(seated["group"]), Create(desk), run_time=1.2)','self.play(FadeIn(room["shell"]), FadeIn(room["glass"]), FadeIn(seated["group"]), FadeIn(desk), run_time=1.2)')
s=s.replace('LaggedStart(FadeIn(laptop, shift=DOWN * 0.15), FadeIn(server, shift=DOWN * 0.15), FadeIn(socket),\n                              lag_ratio=0.3)','LaggedStart(FadeIn(laptop, shift=DOWN * 0.15), FadeIn(server, shift=DOWN * 0.15), lag_ratio=0.3)')
s=s.replace('self.play(Create(cable_lines), Flash(sock,','self.play(Create(cable_lines), Flash(sock,')
s=s.replace('self.play(*device_heat(1.3), laptop.animate.set_color(COLOR_HEAT), lap_w.animate.set_value(PHI_E_LAPTOP_W),\n                  run_time=1.3)','self.play(*device_heat(1.3), Indicate(laptop, color=COLOR_HEAT, scale_factor=1.15), lap_w.animate.set_value(PHI_E_LAPTOP_W),\n                  run_time=1.3)')
s=s.replace('self.play(*device_heat(1.3), server_box.animate.set_color(COLOR_EQUIP), server[1].animate.set_color(COLOR_EQUIP),\n                  dev_w.animate.set_value(PHI_E_DEVICE_W)','self.play(*device_heat(1.3), Indicate(server, color=COLOR_EQUIP, scale_factor=1.15),\n                  dev_w.animate.set_value(PHI_E_DEVICE_W)')
# Beat4
s=s.replace('''        anchor = np.array([room["center"][0], room["y_c"], 0.0])
        lamp = lamp_glyph(anchor, drop=0.45)
        lamp["group"].scale(B4_LAMP_SCALE, about_point=anchor)
        bulb = lamp["bulb"].get_center()''','''        lamp = staffage("heating-internal-gains-pendant-lamp", unit=B4_LAMP_UNIT, anchor=np.array([room["center"][0], room["y_c"], 0.0]))
        lamp.shift(UP * (room["y_c"] - lamp.get_top()[1]))
        bulb = _spot(lamp, 0.5, 0.15)''')
s=s.replace('B4_LAMP_SCALE = 1.8','B4_LAMP_UNIT = 2.4')
s=s.replace('self.play(FadeIn(lamp["group"], shift=DOWN * 0.15), run_time=0.7)','self.play(FadeIn(lamp, shift=DOWN * 0.15), run_time=0.7)')
s=s.replace('np.array([lamp["group"].get_right()[0] + 0.35, lamp["group"].get_bottom()[1] + 0.35, 0.0])','np.array([lamp.get_right()[0] + 0.35, lamp.get_bottom()[1] + 0.35, 0.0])')
s=s.replace('            lamp["bulb"].animate.set_fill(COLOR_LIGHT, opacity=0.85),\n','')
# Beat5
a=s.index("def _b5_icon(")
b=s.index("#endregion",a)
s=s[:a]+'''def _b5_icon(key: str, x: float):
    """🧩 Thermal staffage above its bar — person, laptop or lamp — with the point its heat leaves from."""
    feet = np.array([x, B5_ICON_Y - 0.45, 0.0])
    if key == "p":
        fig = staffage_motion("heating-internal-gains-standing-adult", unit=0.5, anchor=feet)
        return fig, _spot(fig, 0.5, 0.6), False
    if key == "e":
        lap = staffage("heating-internal-gains-laptop", unit=1.1, anchor=feet + UP * 0.1)
        return lap, lap.get_center(), False
    lamp = staffage("heating-internal-gains-pendant-lamp", unit=0.6, anchor=feet)
    lamp.shift(UP * (B5_ICON_Y + 0.5 - lamp.get_top()[1]))
    return lamp, _spot(lamp, 0.5, 0.15), True
'''+s[b:]
s=s.replace('\n',('\r\n' if crlf else '\n'))
import os; open('out.tmp','w',encoding='utf-8',newline='').write(s); os.replace('out.tmp', p)
