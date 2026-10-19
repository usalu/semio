mod inner { pub fn a() {} }
mod other { use super::inner::b; }
