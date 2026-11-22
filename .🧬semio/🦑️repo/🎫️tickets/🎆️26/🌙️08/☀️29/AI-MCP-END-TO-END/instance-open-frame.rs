//! 🐣️ Fixed metadata and row headers for a sole owned-Core InstanceOpen Poll.

pub const MAGIC: [u8; 4] = *b"SPO1";
pub const HEADER_BYTES: usize = 244;
pub const ASSET_HEADER_BYTES: usize = 8;
pub const CAPABILITY_HEADER_BYTES: usize = 36;
pub const QUOTA_COUNT: usize = 17;
pub const LIMIT_COUNT: usize = 11;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct InvalidFrame;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct OpenHeader {
    pub activation_generation: u64,
    pub instance_id: u32,
    pub request_sequence: u64,
    pub limits: [u64; LIMIT_COUNT],
    pub quotas: [Option<u64>; QUOTA_COUNT],
    pub app_bytes: u32,
    pub actor_bytes: u32,
    pub config_bytes: u32,
    pub assets: u32,
    pub capabilities: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AssetHeader {
    pub name_bytes: u32,
    pub data_bytes: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CapabilityHeader {
    pub token: u128,
    pub expires_ms: Option<u64>,
    pub id_bytes: u32,
    pub scope_bytes: u32,
}

const LIMIT_WIDTHS: [usize; LIMIT_COUNT] = [4, 4, 4, 4, 4, 8, 8, 8, 4, 4, 4];

impl OpenHeader {
    pub fn minimum_extent(self) -> Result<usize, InvalidFrame> {
        let fields = [self.app_bytes, self.actor_bytes, self.config_bytes];
        [fields[0] as usize, fields[1] as usize, fields[2] as usize,
            (self.assets as usize).checked_mul(ASSET_HEADER_BYTES).ok_or(InvalidFrame)?,
            (self.capabilities as usize).checked_mul(CAPABILITY_HEADER_BYTES).ok_or(InvalidFrame)?]
            .into_iter().try_fold(HEADER_BYTES, |size, bytes| size.checked_add(bytes).ok_or(InvalidFrame))
    }

    pub fn bytes(self, extent: usize) -> Result<[u8; HEADER_BYTES], InvalidFrame> {
        let extent = checked_extent(extent, self.minimum_extent()?)?;
        let mut header = [0; HEADER_BYTES];
        header[..4].copy_from_slice(&MAGIC);
        put(&mut header, 4, extent.to_le_bytes());
        put(&mut header, 8, self.activation_generation.to_le_bytes());
        put(&mut header, 16, self.instance_id.to_le_bytes());
        put(&mut header, 20, self.request_sequence.to_le_bytes());
        let mut cursor = 28;
        for (value, width) in self.limits.into_iter().zip(LIMIT_WIDTHS) {
            if width == 4 {
                put(&mut header, cursor, u32::try_from(value).map_err(|_| InvalidFrame)?.to_le_bytes());
            } else {
                put(&mut header, cursor, value.to_le_bytes());
            }
            cursor += width;
        }
        let mut presence = 0u32;
        for (index, quota) in self.quotas.into_iter().enumerate() {
            if let Some(quota) = quota {
                presence |= 1 << index;
                put(&mut header, 108 + 8 * index, quota.to_le_bytes());
            }
        }
        put(&mut header, 84, presence.to_le_bytes());
        for (index, value) in [self.app_bytes, self.actor_bytes, self.config_bytes, self.assets, self.capabilities].into_iter().enumerate() {
            put(&mut header, 88 + 4 * index, value.to_le_bytes());
        }
        Ok(header)
    }

    pub fn read(bytes: &[u8], extent: usize) -> Result<Self, InvalidFrame> {
        if bytes.len() != HEADER_BYTES || bytes[..4] != MAGIC {
            return Err(InvalidFrame);
        }
        let presence = u32::from_le_bytes(get(bytes, 84));
        if presence >> QUOTA_COUNT != 0 { return Err(InvalidFrame); }
        let mut quotas = [None; QUOTA_COUNT];
        for (index, field) in quotas.iter_mut().enumerate() {
            let value = u64::from_le_bytes(get(bytes, 108 + 8 * index));
            if presence & (1 << index) != 0 {
                *field = Some(value);
            } else if value != 0 {
                return Err(InvalidFrame);
            }
        }
        let mut cursor = 28;
        let limits = LIMIT_WIDTHS.map(|width| {
            let value = if width == 4 { u64::from(u32::from_le_bytes(get(bytes, cursor))) } else { u64::from_le_bytes(get(bytes, cursor)) };
            cursor += width;
            value
        });
        let header = Self {
            activation_generation: u64::from_le_bytes(get(bytes, 8)),
            instance_id: u32::from_le_bytes(get(bytes, 16)),
            request_sequence: u64::from_le_bytes(get(bytes, 20)),
            limits, quotas,
            app_bytes: u32::from_le_bytes(get(bytes, 88)),
            actor_bytes: u32::from_le_bytes(get(bytes, 92)),
            config_bytes: u32::from_le_bytes(get(bytes, 96)),
            assets: u32::from_le_bytes(get(bytes, 100)),
            capabilities: u32::from_le_bytes(get(bytes, 104)),
        };
        if u32::from_le_bytes(get(bytes, 4)) != checked_extent(extent, header.minimum_extent()?)? {
            return Err(InvalidFrame);
        }
        Ok(header)
    }
}

impl AssetHeader {
    pub fn bytes(self) -> [u8; ASSET_HEADER_BYTES] {
        let mut bytes = [0; ASSET_HEADER_BYTES];
        put(&mut bytes, 0, self.name_bytes.to_le_bytes());
        put(&mut bytes, 4, self.data_bytes.to_le_bytes());
        bytes
    }

    pub fn read(bytes: &[u8]) -> Result<Self, InvalidFrame> {
        if bytes.len() != ASSET_HEADER_BYTES { return Err(InvalidFrame); }
        Ok(Self { name_bytes: u32::from_le_bytes(get(bytes, 0)), data_bytes: u32::from_le_bytes(get(bytes, 4)) })
    }
}

impl CapabilityHeader {
    pub fn bytes(self) -> [u8; CAPABILITY_HEADER_BYTES] {
        let mut bytes = [0; CAPABILITY_HEADER_BYTES];
        put(&mut bytes, 0, self.token.to_le_bytes());
        put(&mut bytes, 16, self.expires_ms.unwrap_or(0).to_le_bytes());
        put(&mut bytes, 24, u32::from(self.expires_ms.is_some()).to_le_bytes());
        put(&mut bytes, 28, self.id_bytes.to_le_bytes());
        put(&mut bytes, 32, self.scope_bytes.to_le_bytes());
        bytes
    }

    pub fn read(bytes: &[u8]) -> Result<Self, InvalidFrame> {
        if bytes.len() != CAPABILITY_HEADER_BYTES { return Err(InvalidFrame); }
        let expiry = u64::from_le_bytes(get(bytes, 16));
        let expires_ms = match u32::from_le_bytes(get(bytes, 24)) {
            0 if expiry == 0 => None,
            1 => Some(expiry),
            _ => return Err(InvalidFrame),
        };
        Ok(Self {
            token: u128::from_le_bytes(get(bytes, 0)), expires_ms,
            id_bytes: u32::from_le_bytes(get(bytes, 28)), scope_bytes: u32::from_le_bytes(get(bytes, 32)),
        })
    }
}

pub fn field(bytes: &[u8], offset: usize, extent: usize) -> Result<&[u8], InvalidFrame> {
    let end = offset.checked_add(extent).ok_or(InvalidFrame)?;
    bytes.get(offset..end).ok_or(InvalidFrame)
}

fn checked_extent(extent: usize, minimum: usize) -> Result<u32, InvalidFrame> {
    if extent < minimum || extent > i32::MAX as usize { return Err(InvalidFrame); }
    Ok(extent as u32)
}

fn put<const LENGTH: usize>(bytes: &mut [u8], offset: usize, value: [u8; LENGTH]) {
    bytes[offset..offset + LENGTH].copy_from_slice(&value);
}

fn get<const LENGTH: usize>(bytes: &[u8], offset: usize) -> [u8; LENGTH] {
    bytes[offset..offset + LENGTH].try_into().unwrap()
}
