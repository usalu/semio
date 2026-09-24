"""🪦️ G10 one-off codemod: `GatewayToShell::ApprovalWithdrawn` (tag 11) in the Rust bridge SSOT — every codec site."""
import pathlib
B = pathlib.Path("/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🧵️bridge/🦀️.rs")
t = B.read_text()
pairs = [
('''/// 📇️ One entry of `Instances{entries}` — `BridgeInstanceRef{plugin_id, app_id, instance_id,''', '''/// 🪦️ Why the gateway took an approval request back before any human decided it. The shell retires
/// the affordance and says why; the gateway no longer waits for a decision on it.
/// - `Cancelled`: the agent's call that parked the request was cancelled.
/// - `TimedOut`: nobody decided within the request's own countdown.
/// - `Superseded`: the request moved to a newer shell connection, which now carries it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "snake_case")]
#[value(rename_all = "snake_case")]
pub enum ApprovalWithdrawal {
    Cancelled,
    TimedOut,
    Superseded,
}

impl ApprovalWithdrawal {
    fn to_tag(self) -> u8 {
        match self {
            ApprovalWithdrawal::Cancelled => 0,
            ApprovalWithdrawal::TimedOut => 1,
            ApprovalWithdrawal::Superseded => 2,
        }
    }
    fn from_tag(tag: u8) -> Result<Self, GatewayError> {
        match tag {
            0 => Ok(ApprovalWithdrawal::Cancelled),
            1 => Ok(ApprovalWithdrawal::TimedOut),
            2 => Ok(ApprovalWithdrawal::Superseded),
            other => Err(GatewayError::new(GatewayErrorCode::InputInvalid, format!("bridge frame: unknown ApprovalWithdrawal tag {other}"))),
        }
    }
}

/// 📇️ One entry of `Instances{entries}` — `BridgeInstanceRef{plugin_id, app_id, instance_id,'''),
('''/// 📤️ Gateway→Shell frames, tag 0..9 in this exact declaration order (`📋️master.md` §2.2).''', '''/// 📤️ Gateway→Shell frames, tag 0..11 in this exact declaration order (`📋️master.md` §2.2); a new
/// variant only ever goes last. Wire fixture: `🧫️fixtures/📨️frames.json`.'''),
('''    AgentReply { reply_id: String, in_reply_to: Option<String>, text: String, complete: bool },
}''', '''    AgentReply { reply_id: String, in_reply_to: Option<String>, text: String, complete: bool },
    /// 🪦️ The approval request `approval_id` is withdrawn — its call was cancelled, it timed out, or it
    /// moved to a newer shell connection — so the shell retires its affordance and says why.
    ApprovalWithdrawn { approval_id: String, reason: ApprovalWithdrawal },
}'''),
('''                in_reply_to.as_ref().map_or(Some(base), |value| base.checked_add(bridge_wire_field_len(value.len())?))
            }
        }
    }''', '''                in_reply_to.as_ref().map_or(Some(base), |value| base.checked_add(bridge_wire_field_len(value.len())?))
            }
            GatewayToShell::ApprovalWithdrawn { approval_id, .. } => 2usize.checked_add(bridge_wire_field_len(approval_id.len())?),
        }
    }'''),
('''                wire::write_string(&mut buf, text);
                wire::write_bool(&mut buf, *complete);
            }
        }
        buf''', '''                wire::write_string(&mut buf, text);
                wire::write_bool(&mut buf, *complete);
            }
            GatewayToShell::ApprovalWithdrawn { approval_id, reason } => {
                wire::write_u8(&mut buf, 11);
                wire::write_string(&mut buf, approval_id);
                wire::write_u8(&mut buf, reason.to_tag());
            }
        }
        buf'''),
('''                writer.field(text.as_bytes());
                writer.push(&[*complete as u8]);
            }
        }
        writer.written''', '''                writer.field(text.as_bytes());
                writer.push(&[*complete as u8]);
            }
            Self::ApprovalWithdrawn { approval_id, reason } => {
                writer.push(&[11]);
                writer.field(approval_id.as_bytes());
                writer.push(&[reason.to_tag()]);
            }
        }
        writer.written'''),
('''            10 => GatewayToShell::AgentReply { reply_id: reader.read_string()?, in_reply_to: reader.read_option_string()?, text: reader.read_string()?, complete: reader.read_bool()? },
            other => return Err(GatewayError::new(GatewayErrorCode::InputInvalid, format!("bridge frame: unknown GatewayToShell tag {other}"))),''', '''            10 => GatewayToShell::AgentReply { reply_id: reader.read_string()?, in_reply_to: reader.read_option_string()?, text: reader.read_string()?, complete: reader.read_bool()? },
            11 => GatewayToShell::ApprovalWithdrawn { approval_id: reader.read_string()?, reason: ApprovalWithdrawal::from_tag(reader.read_u8()?)? },
            other => return Err(GatewayError::new(GatewayErrorCode::InputInvalid, format!("bridge frame: unknown GatewayToShell tag {other}"))),'''),
('''                encoded.write_field(text.as_bytes());
                encoded.write_u8(*complete as u8);
            }
        }
        assert_eq!(encoded.len, expected, "preflighted bridge frame length changed during encode");''', '''                encoded.write_field(text.as_bytes());
                encoded.write_u8(*complete as u8);
            }
            GatewayToShell::ApprovalWithdrawn { approval_id, reason } => {
                encoded.write_u8(11);
                encoded.write_field(approval_id.as_bytes());
                encoded.write_u8(reason.to_tag());
            }
        }
        assert_eq!(encoded.len, expected, "preflighted bridge frame length changed during encode");'''),
]
for old, new in pairs:
    assert t.count(old) == 1, (old[:70], t.count(old))
    t = t.replace(old, new)
B.write_text(t)
Q = pathlib.Path("/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🧵️bridge/🧪️tests/🔬️quick/🦀️.rs")
q = Q.read_text()
old = '''    assert_eq!(gateway_to_shell_count, 13, "fixtures must cover every GatewayToShell variant instance");'''
assert q.count(old) == 1
Q.write_text(q.replace(old, '''    assert_eq!(gateway_to_shell_count, 16, "fixtures must cover every GatewayToShell variant instance");'''))
print("ok")
