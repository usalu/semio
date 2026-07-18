//! 📊 Coordinate axes, number planes, and complex planes.

use crate::color::Color;
use crate::geometry::{arrow, dot, line};
use crate::sobject::{Group, Sobject, VSobject};
use mathematical_geometry::Point;

/// 📈 Cartesian axes with optional labels.
pub struct Axes {
    pub group: Group,
    pub x_length: f64,
    pub y_length: f64,
    pub origin: Point,
}

impl Axes {
    pub fn new(x_length: f64, y_length: f64, origin: Point, color: Color) -> Self {
        let x_axis = arrow(origin, Point::new(origin.x() + x_length, origin.y()), color, 3.0, 0.2);
        let y_axis = arrow(origin, Point::new(origin.x(), origin.y() + y_length), color, 3.0, 0.2);
        let group = Group::new(vec![Box::new(x_axis), Box::new(y_axis)]);
        Self {
            group,
            x_length,
            y_length,
            origin,
        }
    }

    pub fn coords_to_point(&self, x: f64, y: f64) -> Point {
        Point::new(self.origin.x() + x, self.origin.y() + y)
    }

    pub fn as_group(&self) -> &Group {
        &self.group
    }
}

/// 🔲 Number plane with grid lines.
pub struct NumberPlane {
    pub axes: Axes,
    pub group: Group,
    pub unit_size: f64,
}

impl NumberPlane {
    pub fn new(x_range: (f64, f64), y_range: (f64, f64), unit_size: f64, color: Color) -> Self {
        let origin = Point::new(-x_range.0 * unit_size, -y_range.0 * unit_size);
        let x_len = (x_range.1 - x_range.0) * unit_size;
        let y_len = (y_range.1 - y_range.0) * unit_size;
        let axes = Axes::new(x_len, y_len, origin, color);
        let mut children: Vec<Box<dyn Sobject>> = vec![
            Box::new(arrow(origin, Point::new(origin.x() + x_len, origin.y()), color, 3.0, 0.2)),
            Box::new(arrow(origin, Point::new(origin.x(), origin.y() + y_len), color, 3.0, 0.2)),
        ];
        let grid_color = color.with_alpha(0.25);
        let x_steps = ((x_range.1 - x_range.0) as i32).abs().max(1) as i32;
        let y_steps = ((y_range.1 - y_range.0) as i32).abs().max(1) as i32;
        for i in 0..=x_steps {
            let x = origin.x() + i as f64 * unit_size;
            children.push(Box::new(line(
                Point::new(x, origin.y()),
                Point::new(x, origin.y() + y_len),
                grid_color,
                1.0,
            )));
        }
        for j in 0..=y_steps {
            let y = origin.y() + j as f64 * unit_size;
            children.push(Box::new(line(
                Point::new(origin.x(), y),
                Point::new(origin.x() + x_len, y),
                grid_color,
                1.0,
            )));
        }
        let group = Group::new(children);
        Self {
            axes,
            group,
            unit_size,
        }
    }
}

/// ➖ One-dimensional number line.
pub struct NumberLine {
    pub group: Group,
    pub start: Point,
    pub length: f64,
}

impl NumberLine {
    pub fn new(start: Point, length: f64, color: Color) -> Self {
        let axis = line(start, Point::new(start.x() + length, start.y()), color, 3.0);
        let tick_count = 10;
        let mut children: Vec<Box<dyn Sobject>> = vec![Box::new(axis)];
        for i in 0..=tick_count {
            let x = start.x() + length * i as f64 / tick_count as f64;
            children.push(Box::new(line(
                Point::new(x, start.y() - 0.1),
                Point::new(x, start.y() + 0.1),
                color,
                1.5,
            )));
        }
        Self {
            group: Group::new(children),
            start,
            length,
        }
    }

    pub fn number_to_point(&self, n: f64) -> Point {
        Point::new(self.start.x() + n, self.start.y())
    }
}

/// 🔢 Integer-only number line with unit ticks.
pub struct IntegerLine {
    pub group: Group,
    pub start: Point,
    pub unit_size: f64,
    pub min: i32,
    pub max: i32,
}

impl IntegerLine {
    pub fn new(start: Point, min: i32, max: i32, unit_size: f64, color: Color) -> Self {
        let span = (max - min).max(1) as f64;
        let length = span * unit_size;
        let axis = line(start, Point::new(start.x() + length, start.y()), color, 3.0);
        let mut children: Vec<Box<dyn Sobject>> = vec![Box::new(axis)];
        for value in min..=max {
            let x = start.x() + (value - min) as f64 * unit_size;
            children.push(Box::new(line(
                Point::new(x, start.y() - 0.12),
                Point::new(x, start.y() + 0.12),
                color,
                1.5,
            )));
            if value % 5 == 0 {
                children.push(Box::new(dot(Point::new(x, start.y()), 0.04, color)));
            }
        }
        Self {
            group: Group::new(children),
            start,
            unit_size,
            min,
            max,
        }
    }

    pub fn integer_to_point(&self, n: i32) -> Point {
        Point::new(self.start.x() + (n - self.min) as f64 * self.unit_size, self.start.y())
    }
}

/// ℂ Complex plane (axes with imaginary vertical axis).
pub struct ComplexPlane {
    pub plane: NumberPlane,
}

impl ComplexPlane {
    pub fn new(range: f64, unit_size: f64, color: Color) -> Self {
        let plane = NumberPlane::new((-range, range), (-range, range), unit_size, color);
        Self { plane }
    }

    pub fn complex_to_point(&self, re: f64, im: f64) -> Point {
        self.plane.axes.coords_to_point(re * self.plane.unit_size, im * self.plane.unit_size)
    }

    pub fn plot_point(&self, re: f64, im: f64, color: Color) -> VSobject {
        dot(self.complex_to_point(re, im), 0.06, color)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn axes_map_coordinates() {
        let axes = Axes::new(4.0, 3.0, Point::ZERO, Color::WHITE);
        let p = axes.coords_to_point(1.0, 2.0);
        assert!((p.x() - 1.0).abs() < 1e-9);
        assert!((p.y() - 2.0).abs() < 1e-9);
    }

    #[test]
    fn number_line_maps_values() {
        let nl = NumberLine::new(Point::ZERO, 10.0, Color::WHITE);
        assert!((nl.number_to_point(5.0).x() - 5.0).abs() < 1e-9);
    }

    #[test]
    fn integer_line_maps_values() {
        let il = IntegerLine::new(Point::ZERO, 0, 10, 1.0, Color::WHITE);
        assert!((il.integer_to_point(5).x() - 5.0).abs() < 1e-9);
    }
}
