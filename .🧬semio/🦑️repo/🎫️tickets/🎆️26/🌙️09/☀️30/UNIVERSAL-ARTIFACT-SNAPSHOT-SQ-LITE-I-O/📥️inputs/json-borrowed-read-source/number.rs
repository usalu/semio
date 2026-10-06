        let length=self.position-self.start;
        let value=if length<=1200{
            let mut text=borrowed_read_source::NumberText::new();
            for position in self.start..self.position{text.push(input.byte_at(position).ok_or(JsonError::InvalidNumber(self.start))?)?;}
            text.text()?.parse::<f64>()
        }else if self.significant==0{Ok(if self.negative{-0.0}else{0.0})}
        else{
            let mut normalized=borrowed_read_source::NumberText::new();if self.negative{normalized.push(b'-')?;}
            let mut first=true;
            for position in self.significant_start..self.significant_end{
                let digit=input.byte_at(position).ok_or(JsonError::InvalidNumber(self.start))?;if digit==b'.'{continue;}
                normalized.push(digit)?;if first{normalized.push(b'.')?;first=false;}
            }
            if self.sticky{normalized.push(b'1')?;}normalized.push(b'e')?;
            let exponent=if self.exponent_negative{-self.exponent}else{self.exponent};
            std::fmt::Write::write_fmt(&mut normalized,format_args!("{}",self.integer_digits.saturating_sub(self.leading).saturating_sub(1).saturating_add(exponent))).map_err(|_|JsonError::InvalidNumber(self.start))?;
            normalized.text()?.parse::<f64>()
        }.map_err(|_|JsonError::InvalidNumber(self.start))?;
