//! 🧮️ Unmounted TIFF reconstruction backing admission for explicit typed collections.
use semio_framework_os_kernel::sqlite_snapshot::{SqliteRow,SqliteSnapshotControl,SqliteSnapshotPhase};

fn owned_vec<T>(count:usize,control:&mut SqliteSnapshotControl<'_>)->Result<Vec<T>,String>{
    let bytes=count.checked_mul(std::mem::size_of::<T>()).ok_or("TIFF construction backing overflow")?;
    control.admit_allocation_bytes(bytes)?;
    let mut output=Vec::new();output.try_reserve_exact(count).map_err(|_|"TIFF construction allocation failed")?;Ok(output)
}
fn typed_values<T>(rows:&[&SqliteRow],control:&mut SqliteSnapshotControl<'_>,read:impl Fn(&SqliteRow)->Result<T,String>)->Result<Vec<T>,String>{
    control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,0,rows.len())?;
    let mut output=owned_vec(rows.len(),control)?;
    for(index,row)in rows.iter().enumerate(){output.push(read(row)?);if(index+1)%256==0{control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,index+1,rows.len())?;}}
    control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,rows.len(),rows.len())?;Ok(output)
}
fn borrowed_rows<'a>(rows:impl ExactSizeIterator<Item=&'a SqliteRow>,control:&mut SqliteSnapshotControl<'_>)->Result<Vec<&'a SqliteRow>,String>{
    let count=rows.len();let mut output=owned_vec(count,control)?;
    for(index,row)in rows.enumerate(){output.push(row);if(index+1)%256==0{control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,index+1,count)?;}}
    control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,count,count)?;Ok(output)
}
