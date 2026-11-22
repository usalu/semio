//! 📤️ A borrowed InstanceOpen exposes schema-ordered spans from its actual kernel event.
use super::{ASSET_HEADER_BYTES, AssetHeader, CAPABILITY_HEADER_BYTES, CapabilityHeader, HEADER_BYTES, OpenHeader};
use semio_framework::kernel::{Event, OwnedCorePollLimits};
use semio_framework_value::{ValueError, ValueRefusalKind};

#[derive(Clone, Copy)]
pub struct OpenView<'a> {
    event: &'a Event,
    limits: OwnedCorePollLimits,
}

pub enum OpenSegment<'a> {
    Header([u8; HEADER_BYTES]),
    Asset([u8; ASSET_HEADER_BYTES]),
    Capability([u8; CAPABILITY_HEADER_BYTES]),
    Borrowed(&'a [u8]),
}

impl OpenSegment<'_> {
    pub fn bytes(&self) -> &[u8] {
        match self {
            Self::Header(bytes)=>bytes, Self::Asset(bytes)=>bytes, Self::Capability(bytes)=>bytes, Self::Borrowed(bytes)=>bytes,
        }
    }
}

impl<'a> OpenView<'a> {
    pub fn select(event: &'a Event, limits: OwnedCorePollLimits) -> Option<Self> {
        matches!(event,Event::InstanceOpen{..}).then_some(Self{event,limits})
    }

    pub fn identity(self) -> (usize, OwnedCorePollLimits) {
        (std::ptr::from_ref(self.event) as usize,self.limits)
    }

    pub fn segments(self) -> Result<usize, ValueError> {
        let Event::InstanceOpen{assets,capabilities,..}=self.event else{return Err(invalid())};
        assets.len().checked_add(capabilities.len()).and_then(|rows|rows.checked_mul(3))
            .and_then(|body|body.checked_add(4)).ok_or_else(ownership_limit)
    }

    pub fn segment(self, index: usize, extent: usize) -> Result<OpenSegment<'a>, ValueError> {
        let Event::InstanceOpen{request,app_id,actor,config,assets,capabilities,quotas}=self.event else{return Err(invalid())};
        match index {
            0=>{
                if !request.is_valid(){return Err(invalid())}
                let header=OpenHeader{
                    activation_generation:request.activation_generation,instance_id:request.instance_id,request_sequence:request.request_sequence,
                    limits:std::array::from_fn(|index|self.limits.scalar_at(index).unwrap()),
                    quotas:[quotas.memory_bytes,quotas.fuel_per_turn,quotas.turn_deadline_ms,quotas.tables,quotas.mailbox_len,
                        quotas.message_bytes,quotas.outstanding_requests,quotas.timers,quotas.storage_bytes,quotas.network_bytes_per_min,
                        quotas.ui_nodes,quotas.patch_bytes,quotas.patch_hz,quotas.blob_resident_bytes,quotas.gpu_ms_per_frame,
                        quotas.background_ms_per_min,quotas.log_bytes_per_min],
                    app_bytes:width(app_id.0.len())?,actor_bytes:width(actor.len())?,config_bytes:width(config.len())?,
                    assets:width(assets.len())?,capabilities:width(capabilities.len())?,
                };
                Ok(OpenSegment::Header(header.bytes(extent).map_err(|_|ownership_limit())?))
            }
            1=>Ok(OpenSegment::Borrowed(app_id.0.as_bytes())),
            2=>Ok(OpenSegment::Borrowed(actor.as_bytes())),
            3=>Ok(OpenSegment::Borrowed(config)),
            _=>{
                let row=(index-4)/3;
                let field=(index-4)%3;
                if let Some((name,data))=assets.get(row){
                    return Ok(match field {
                        0=>OpenSegment::Asset(AssetHeader{name_bytes:width(name.len())?,data_bytes:width(data.len())?}.bytes()),
                        1=>OpenSegment::Borrowed(name.as_bytes()),
                        _=>OpenSegment::Borrowed(data),
                    })
                }
                let row=capabilities.get(row-assets.len()).ok_or_else(invalid)?;
                Ok(match field{
                    0=>OpenSegment::Capability(CapabilityHeader{token:row.token.0,expires_ms:row.expires_ms,id_bytes:width(row.id.0.len())?,scope_bytes:width(row.scope.len())?}.bytes()),
                    1=>OpenSegment::Borrowed(row.id.0.as_bytes()),
                    _=>OpenSegment::Borrowed(row.scope.as_bytes()),
                })
            }
        }
    }
}

fn width(value:usize)->Result<u32,ValueError>{u32::try_from(value).map_err(|_|ownership_limit())}
fn ownership_limit()->ValueError{ValueError::literal(ValueRefusalKind::OwnershipLimit,"compact InstanceOpen exceeds its Core extent")}
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvalidValue,"compact InstanceOpen source is invalid")}
