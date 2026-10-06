//! 🚦️ Admitted borrowed relational frontiers and bounded literal ordering.
use super::close;
use semio_framework_value::{NativeDecodeControl, ValueError, ValueRefusalKind};
use std::cmp::Ordering;

pub(crate) fn compare_literal(left: &str, right: &str, control: &mut NativeDecodeControl<'_>) -> Result<Ordering, ValueError> {
    control.scoped_stage(|control| {
        let length = left.len().min(right.len());
        control.begin_stage(length)?;
        let mut position = 0;
        while position < length {
            let end = position.saturating_add(65536).min(length);
            let order = left.as_bytes()[position..end].cmp(&right.as_bytes()[position..end]);
            control.advance(end - position)?;
            if order != Ordering::Equal { return Ok(order); }
            position = end;
        }
        control.checkpoint()?;
        Ok(left.len().cmp(&right.len()))
    })
}

fn sift<T>(values: &mut [T], mut root: usize, end: usize, compare: &mut impl FnMut(&T, &T, &mut NativeDecodeControl<'_>) -> Result<Ordering, ValueError>, control: &mut NativeDecodeControl<'_>) -> Result<(), ValueError> {
    loop {
        let Some(child) = root.checked_mul(2).and_then(|value| value.checked_add(1)).filter(|child| *child < end) else { return Ok(()); };
        let selected = if child + 1 < end && compare(&values[child], &values[child + 1], control)? == Ordering::Less { child + 1 } else { child };
        if compare(&values[root], &values[selected], control)? != Ordering::Less { return Ok(()); }
        values.swap(root, selected); control.step()?; root = selected;
    }
}

pub(crate) fn order<T>(values: &mut [T], mut compare: impl FnMut(&T, &T, &mut NativeDecodeControl<'_>) -> Result<Ordering, ValueError>, control: &mut NativeDecodeControl<'_>) -> Result<(), ValueError> {
    control.scoped_stage(|control| {
        let height=usize::BITS as usize-values.len().leading_zeros() as usize;
        let work=values.len().checked_mul(2).and_then(|count|count.checked_mul(height+1)).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"CAD sorted frontier work count overflow"))?;
        control.begin_stage(work)?;
        for root in (0..values.len() / 2).rev() { sift(values, root, values.len(), &mut compare, control)?; control.step()?; }
        for end in (1..values.len()).rev() { values.swap(0, end); sift(values, 0, end, &mut compare, control)?; control.step()?; }
        control.checkpoint()
    })
}









pub(crate) fn collect<T: semio_framework_value::retirement::RetireOwned>(count: usize, control: &mut NativeDecodeControl<'_>, mut item: impl FnMut(usize, &mut NativeDecodeControl<'_>) -> Result<T, ValueError>) -> Result<Vec<T>, ValueError> {
    control.scoped_stage(|control| {
        control.begin_stage(count)?;
        let mut output = semio_framework_dsl_record::__rt::DecodedFieldOwner::new(control.allocate_vec(count)?, close::<Vec<T>>);
        for index in 0..count { output.as_mut().push(control.scoped_stage(|control| item(index, control))?); control.step()?; }
        Ok(output.take())
    })
}
